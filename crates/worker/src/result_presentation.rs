//! Model-facing text for fixed-tool results.
//!
//! The ledger keeps every `tool_result` as the backend's canonical JSON; that
//! record is what validation, edit recording, and replay read. The provider
//! request carries this projection instead, so the model reads a file as
//! numbered lines and a command as its output rather than as escaped JSON.
//! Unknown tools and non-JSON bodies pass through unchanged.

use serde_json::Value;

/// Plain text for a fixed tool's result body, or `None` when the body is not
/// one this projection owns.
pub(crate) fn present(name: &str, text: &str, ok: bool) -> Option<String> {
    let value: Value = serde_json::from_str(text).ok()?;
    let object = value.as_object()?;
    if !ok {
        if let Some(reason) = object.get("denied").and_then(Value::as_str) {
            return Some(format!("denied: {reason}"));
        }
        if let (Some(code), Some(message)) = (
            object.get("code").and_then(Value::as_str),
            object.get("message").and_then(Value::as_str),
        ) {
            return Some(format!("error ({code}): {message}"));
        }
        return None;
    }
    match name {
        "read" => present_read(object),
        "shell" => present_shell(object),
        "grep" => present_grep(object),
        "glob" => present_glob(object),
        "write" => Some(format!(
            "Wrote {} ({} bytes, artifact_version {})",
            str_field(object, "path")?,
            u64_field(object, "bytes")?,
            u64_field(object, "artifact_version")?
        )),
        "edit" => Some(format!(
            "Edited {} ({} bytes, artifact_version {})",
            str_field(object, "path")?,
            u64_field(object, "bytes")?,
            u64_field(object, "artifact_version")?
        )),
        "apply_patch" => Some(format!(
            "Patched {}: +{} -{} lines, artifact_version {}",
            str_field(object, "path")?,
            u64_field(object, "added_lines")?,
            u64_field(object, "removed_lines")?,
            u64_field(object, "artifact_version")?
        )),
        _ => None,
    }
}

fn str_field<'a>(object: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a str> {
    object.get(key).and_then(Value::as_str)
}

fn u64_field(object: &serde_json::Map<String, Value>, key: &str) -> Option<u64> {
    object.get(key).and_then(Value::as_u64)
}

fn present_read(object: &serde_json::Map<String, Value>) -> Option<String> {
    let lines = object.get("lines")?.as_array()?;
    let total = u64_field(object, "total_lines")?;
    let version = u64_field(object, "artifact_version")?;
    let mut out = String::new();
    let mut first = None;
    let mut last = 0;
    for line in lines {
        let number = line.get("line").and_then(Value::as_u64)?;
        let text = line.get("text").and_then(Value::as_str)?;
        first.get_or_insert(number);
        last = number;
        out.push_str(&format!("{number}: {text}"));
        if line.get("truncated").and_then(Value::as_bool) == Some(true) {
            out.push_str(" [line truncated]");
        }
        out.push('\n');
    }
    match first {
        None => out.push_str(&format!("[empty range; file has {total} lines, artifact_version {version}]\n")),
        Some(first) if object.get("truncated").and_then(Value::as_bool) == Some(true) => out.push_str(&format!(
            "[lines {first}-{last} of {total}; continue with offset {}; artifact_version {version}]\n",
            last + 1
        )),
        Some(first) => out.push_str(&format!("[lines {first}-{last} of {total}; artifact_version {version}]\n")),
    }
    Some(out)
}

/// A helper byte string: UTF-8 text verbatim, binary named by size.
fn bytes_text(value: Option<&Value>) -> String {
    let Some(value) = value else {
        return String::new();
    };
    let data = value
        .get("data")
        .and_then(Value::as_str)
        .unwrap_or_default();
    match value.get("encoding").and_then(Value::as_str) {
        Some("base64") if !data.is_empty() => {
            format!("[{} base64 bytes of non-UTF-8 output]", data.len())
        }
        _ => data.to_owned(),
    }
}

/// One exec step's output: stdout, then stderr under its own marker, then
/// truncation and a non-zero exit code.
fn exec_text(object: &serde_json::Map<String, Value>) -> String {
    let mut out = String::new();
    let stdout = bytes_text(object.get("stdout"));
    let stderr = bytes_text(object.get("stderr"));
    out.push_str(&stdout);
    if object.get("stdout_truncated").and_then(Value::as_bool) == Some(true) {
        if !out.ends_with('\n') && !out.is_empty() {
            out.push('\n');
        }
        out.push_str("[stdout truncated]\n");
    }
    if !stderr.is_empty() {
        if !out.ends_with('\n') && !out.is_empty() {
            out.push('\n');
        }
        out.push_str("[stderr]\n");
        out.push_str(&stderr);
        if object.get("stderr_truncated").and_then(Value::as_bool) == Some(true) {
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("[stderr truncated]\n");
        }
    }
    let code = object.get("exit_code").and_then(Value::as_i64).unwrap_or(0);
    if code != 0 {
        if !out.ends_with('\n') && !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&format!("[exit code: {code}]\n"));
    }
    out
}

fn present_shell(object: &serde_json::Map<String, Value>) -> Option<String> {
    let steps = object.get("steps")?.as_array()?;
    let mut out = String::new();
    for step in steps {
        let step = step.as_object()?;
        if steps.len() > 1 {
            out.push_str(&format!(
                "[step {} {}]\n",
                step.get("index")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
                step.get("program")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            ));
        }
        match step.get("status").and_then(Value::as_str) {
            Some("not_executed") => {
                if step.get("budget_exhausted").and_then(Value::as_bool) == Some(true) {
                    out.push_str(&format!(
                        "[not executed: the {} ms time budget was used up by earlier steps]\n",
                        step.get("budget_ms")
                            .and_then(Value::as_u64)
                            .unwrap_or_default()
                    ));
                } else {
                    out.push_str("[not executed]\n");
                }
                continue;
            }
            Some("timed_out") => {
                out.push_str(&timed_out_text(step));
                continue;
            }
            _ => {}
        }
        out.push_str(&exec_text(step));
    }
    if let Some(artifacts) = object.get("artifacts").and_then(Value::as_array) {
        for artifact in artifacts {
            let path = artifact
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or_default();
            match artifact.get("status").and_then(Value::as_str) {
                Some("verified") => out.push_str(&format!(
                    "[artifact {path}: {} bytes, artifact_version {}]\n",
                    artifact
                        .get("bytes")
                        .and_then(Value::as_u64)
                        .unwrap_or_default(),
                    artifact
                        .get("artifact_version")
                        .and_then(Value::as_u64)
                        .unwrap_or_default()
                )),
                Some(status) => out.push_str(&format!("[artifact {path}: {status}]\n")),
                None => {}
            }
        }
    }
    if out.is_empty() {
        out.push_str("(no output)\n");
    }
    Some(out)
}

/// A step the helper killed at its deadline. Names the budget that applied
/// and how it was chosen, so the model can raise `max_duration_ms` up to the
/// cap or move the work to a detached `job` instead of retrying blindly.
fn timed_out_text(step: &serde_json::Map<String, Value>) -> String {
    let killed_after = step
        .get("killed_after_ms")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let cap = step
        .get("max_duration_ms")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let source = match step.get("budget_source").and_then(Value::as_str) {
        Some("default") => "the default budget because max_duration_ms was omitted".to_owned(),
        Some("clamped") => format!("max_duration_ms clamped to the {cap} ms cap"),
        _ => "the requested max_duration_ms".to_owned(),
    };
    format!(
        "[killed after {killed_after} ms: the command exceeded {source}; no output was kept. \
         Re-run with max_duration_ms up to {cap} ms, or start it as a job for a longer run]\n"
    )
}

fn present_grep(object: &serde_json::Map<String, Value>) -> Option<String> {
    let matched = object.get("matched").and_then(Value::as_bool)?;
    if !matched {
        let stderr = bytes_text(object.get("stderr"));
        return Some(if stderr.is_empty() {
            "[no matches]\n".to_owned()
        } else {
            format!("[no matches]\n[stderr]\n{stderr}")
        });
    }
    Some(exec_text(object))
}

fn present_glob(object: &serde_json::Map<String, Value>) -> Option<String> {
    let paths = object.get("paths")?.as_array()?;
    if paths.is_empty() {
        return Some("[no matches]\n".to_owned());
    }
    let mut out = String::new();
    for path in paths {
        out.push_str(path.as_str()?);
        out.push('\n');
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn text(value: Value) -> String {
        serde_json_canonicalizer::to_string(&value).unwrap()
    }

    #[test]
    fn read_renders_numbered_lines_and_a_continuation_trailer() {
        let body = text(
            json!({"path":"a.py","offset":1,"limit":2,"total_lines":5,"bytes":40,
            "sha256":"sha256-x","artifact_version":3,"truncated":true,
            "lines":[{"line":1,"text":"import os","truncated":false},{"line":2,"text":"x = 1","truncated":true}]}),
        );
        assert_eq!(
            present("read", &body, true).unwrap(),
            "1: import os\n2: x = 1 [line truncated]\n[lines 1-2 of 5; continue with offset 3; artifact_version 3]\n"
        );
        let whole = text(
            json!({"path":"a.py","offset":1,"limit":2000,"total_lines":1,"bytes":2,
            "sha256":"s","artifact_version":1,"truncated":false,"lines":[{"line":1,"text":"hi","truncated":false}]}),
        );
        assert_eq!(
            present("read", &whole, true).unwrap(),
            "1: hi\n[lines 1-1 of 1; artifact_version 1]\n"
        );
    }

    #[test]
    fn shell_renders_output_stderr_and_nonzero_exit_only() {
        let ok = text(
            json!({"artifacts":[],"is_error":false,"status":"completed","steps":[{"exit_code":0,"index":0,
            "program":"/bin/sh","status":"completed","stdout":{"data":"hello\n","encoding":"utf8"},"stdout_truncated":false,
            "stderr":{"data":"","encoding":"utf8"},"stderr_truncated":false}]}),
        );
        assert_eq!(present("shell", &ok, true).unwrap(), "hello\n");
        let failed = text(
            json!({"artifacts":[{"path":"out.txt","status":"not_checked"}],"is_error":true,"status":"failed","steps":[{"exit_code":2,"index":0,
            "program":"/bin/sh","status":"failed","stdout":{"data":"partial","encoding":"utf8"},"stdout_truncated":true,
            "stderr":{"data":"boom\n","encoding":"utf8"},"stderr_truncated":false},{"index":1,"program":"pytest","status":"not_executed"}]}),
        );
        assert_eq!(
            present("shell", &failed, true).unwrap(),
            "[step 0 /bin/sh]\npartial\n[stdout truncated]\n[stderr]\nboom\n[exit code: 2]\n[step 1 pytest]\n[not executed]\n[artifact out.txt: not_checked]\n"
        );
        let silent = text(
            json!({"artifacts":[],"is_error":false,"status":"completed","steps":[{"exit_code":0,"index":0,
            "program":"/bin/sh","status":"completed","stdout":{"data":"","encoding":"utf8"},"stdout_truncated":false,
            "stderr":{"data":"","encoding":"utf8"},"stderr_truncated":false}]}),
        );
        assert_eq!(present("shell", &silent, true).unwrap(), "(no output)\n");
    }

    #[test]
    fn shell_names_a_killed_step_and_the_budget_that_applied() {
        let killed = text(
            json!({"artifacts":[],"is_error":true,"status":"failed","steps":[
            {"index":0,"program":"/bin/sh","status":"timed_out","killed_after_ms":300000,"budget_ms":300000,
             "budget_source":"default","max_duration_ms":600000},
            {"index":1,"program":"pytest","status":"not_executed"}]}),
        );
        assert_eq!(
            present("shell", &killed, true).unwrap(),
            "[step 0 /bin/sh]\n[killed after 300000 ms: the command exceeded the default budget because max_duration_ms was omitted; no output was kept. Re-run with max_duration_ms up to 600000 ms, or start it as a job for a longer run]\n[step 1 pytest]\n[not executed]\n"
        );
        let clamped = text(
            json!({"artifacts":[],"is_error":true,"status":"failed","steps":[
            {"index":0,"program":"/bin/sh","status":"timed_out","killed_after_ms":600000,"budget_ms":600000,
             "budget_source":"clamped","max_duration_ms":600000}]}),
        );
        assert_eq!(
            present("shell", &clamped, true).unwrap(),
            "[killed after 600000 ms: the command exceeded max_duration_ms clamped to the 600000 ms cap; no output was kept. Re-run with max_duration_ms up to 600000 ms, or start it as a job for a longer run]\n"
        );
        let exhausted = text(
            json!({"artifacts":[],"is_error":true,"status":"failed","steps":[
            {"index":0,"program":"/bin/sh","status":"timed_out","killed_after_ms":50,"budget_ms":200,
             "budget_source":"requested","max_duration_ms":600000},
            {"index":1,"program":"pytest","status":"not_executed","budget_exhausted":true,"budget_ms":200}]}),
        );
        assert!(present("shell", &exhausted, true).unwrap().ends_with(
            "[step 1 pytest]\n[not executed: the 200 ms time budget was used up by earlier steps]\n"
        ));
    }

    #[test]
    fn grep_glob_and_edits_render_compactly() {
        let hit = text(
            json!({"exit_code":0,"matched":true,"status":"completed","stdout":{"data":"a.py:3:x = 1\n","encoding":"utf8"},
            "stdout_truncated":false,"stderr":{"data":"","encoding":"utf8"},"stderr_truncated":false}),
        );
        assert_eq!(present("grep", &hit, true).unwrap(), "a.py:3:x = 1\n");
        let miss = text(
            json!({"exit_code":1,"matched":false,"status":"completed","stdout":{"data":"","encoding":"utf8"},
            "stdout_truncated":false,"stderr":{"data":"","encoding":"utf8"},"stderr_truncated":false}),
        );
        assert_eq!(present("grep", &miss, true).unwrap(), "[no matches]\n");
        assert_eq!(
            present("glob", &text(json!({"paths":[]})), true).unwrap(),
            "[no matches]\n"
        );
        assert_eq!(
            present("glob", &text(json!({"paths":["a.py","b/c.py"]})), true).unwrap(),
            "a.py\nb/c.py\n"
        );
        let written = text(json!({"path":"a.py","bytes":12,"sha256":"s","artifact_version":1}));
        assert_eq!(
            present("write", &written, true).unwrap(),
            "Wrote a.py (12 bytes, artifact_version 1)"
        );
        assert_eq!(
            present("edit", &written, true).unwrap(),
            "Edited a.py (12 bytes, artifact_version 1)"
        );
        let patched = text(
            json!({"path":"a.py","summary":"s","artifact_version":2,"bytes":12,"sha256":"s","added_lines":3,"removed_lines":1,"fuzz":0}),
        );
        assert_eq!(
            present("apply_patch", &patched, true).unwrap(),
            "Patched a.py: +3 -1 lines, artifact_version 2"
        );
    }

    #[test]
    fn errors_and_foreign_bodies_are_left_to_the_caller() {
        let error = text(
            json!({"code":"conflict","message":"patch context did not match","retryable":false}),
        );
        assert_eq!(
            present("apply_patch", &error, false).unwrap(),
            "error (conflict): patch context did not match"
        );
        assert_eq!(
            present("mcp__x__y", &text(json!({"denied":"policy"})), false).unwrap(),
            "denied: policy"
        );
        // An ok body with an error-like shape belongs to the tool, not to us.
        assert_eq!(present("mcp__x__y", &error, true), None);
        assert_eq!(present("read", "not json", true), None);
        assert_eq!(
            present("shell", &text(json!({"unexpected":true})), true),
            None
        );
    }
}
