use super::*;

const SELECTED_SUBKIND: &str = "identity.selected";

/// Fixed genesis identities and legacy sessions need no new event. An auto
/// session records its choice before the first provider epoch is assembled.
pub(super) fn selected(
    ledger: &LockedLedger,
) -> Result<Option<tools::IdentityProfile>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let genesis = projection.events.first().ok_or("genesis missing")?;
    match genesis.string_field("identity_profile") {
        None | Some("coding") => Ok(Some(tools::IdentityProfile::Coding)),
        Some("general") => Ok(Some(tools::IdentityProfile::General)),
        Some("auto") => {
            let mut choice = None;
            for event in &projection.events {
                if event.kind() != &EventKind::State
                    || event.string_field("subkind") != Some(SELECTED_SUBKIND)
                {
                    continue;
                }
                if choice.is_some() {
                    return Err("duplicate identity selection".into());
                }
                let raw = serde_json::to_value(event.raw())?;
                let profile = raw
                    .get("payload")
                    .and_then(|payload| payload.get("profile"))
                    .and_then(Value::as_str)
                    .and_then(tools::IdentityProfile::parse)
                    .ok_or("invalid identity selection")?;
                choice = Some(profile);
            }
            Ok(choice)
        }
        _ => Err("invalid session identity profile".into()),
    }
}

pub(super) fn resolve(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
) -> Result<tools::IdentityProfile, Box<dyn std::error::Error>> {
    if let Some(profile) = selected(ledger)? {
        return Ok(profile);
    }
    let input_seq = *current_turn_inputs(ledger, turn)?
        .first()
        .ok_or("auto identity requires a user input")?;
    let first_input = ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
        .iter()
        .find(|event| event.kind() == &EventKind::Input && event.seq() == input_seq)
        .ok_or("auto identity requires a user input")?;
    let raw = serde_json::to_value(first_input.raw())?;
    let content = materialize_json(
        ledger,
        raw.get("content").ok_or("first input lacks content")?,
    )?;
    let text = content
        .as_array()
        .ok_or("first input content is not an array")?
        .iter()
        .filter_map(|block| {
            (block.get("type").and_then(Value::as_str) == Some("text"))
                .then(|| block.get("text").and_then(Value::as_str))
                .flatten()
        })
        .collect::<Vec<_>>()
        .join("\n");
    let profile = classify(&text);
    let seq = ledger.next_seq();
    ledger.append_contract(
        make_event(json!({
            "v":1,"seq":seq,"turn":turn,"kind":"state",
            "ts":timestamp,"subkind":SELECTED_SUBKIND,
            "payload":{"profile":profile.as_str()},"visibility":"runtime"
        }))?,
        BarrierContext::default(),
    )?;
    Ok(profile)
}

/// Deliberately conservative: code signals win in mixed requests. The
/// classifier uses only the first user's text, never a model call or file
/// contents. Ambiguous imperative tasks retain the established coding role.
fn classify(text: &str) -> tools::IdentityProfile {
    let lower = text.to_lowercase();
    let words = lower
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    let code_words = [
        "code",
        "coding",
        "repo",
        "repository",
        "bug",
        "test",
        "tests",
        "compile",
        "build",
        "script",
        "function",
        "api",
        "rust",
        "python",
        "typescript",
        "swift",
        "javascript",
        "git",
        "terminal",
        "shell",
        "implement",
        "refactor",
        "debug",
        "patch",
        "kernel",
        "app",
        "website",
        "sdk",
        "cli",
        "commit",
        "pr",
    ];
    let code_han = [
        "代码",
        "编译",
        "测试",
        "仓库",
        "脚本",
        "函数",
        "接口",
        "报错",
        "修复",
        "重构",
        "实现",
        "开发",
        "程序",
        "命令",
        "提交",
        "模型调用",
        "工具调用",
    ];
    let code_suffixes = [
        ".rs", ".py", ".swift", ".ts", ".tsx", ".js", ".jsx", ".go", ".sh", ".html", ".css",
        ".json", ".toml", ".yaml",
    ];
    if words.iter().any(|word| code_words.contains(word))
        || code_han.iter().any(|term| lower.contains(term))
        || code_suffixes.iter().any(|suffix| lower.contains(suffix))
        || lower.contains("```")
    {
        return tools::IdentityProfile::Coding;
    }
    let general_words = [
        "translate",
        "translation",
        "summarize",
        "summary",
        "research",
        "explain",
        "story",
        "poem",
        "email",
        "letter",
        "essay",
        "brainstorm",
        "recommend",
        "compare",
        "what",
        "why",
        "how",
    ];
    let general_han = [
        "翻译",
        "总结",
        "摘要",
        "调研",
        "解释",
        "故事",
        "诗",
        "邮件",
        "文案",
        "推荐",
        "比较",
        "是什么",
        "为什么",
        "怎么样",
        "如何",
        "分析",
    ];
    if words.iter().any(|word| general_words.contains(word))
        || general_han.iter().any(|term| lower.contains(term))
    {
        tools::IdentityProfile::General
    } else {
        tools::IdentityProfile::Coding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auto_ledger(first: &str) -> (tempfile::TempDir, LockedLedger) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("main.jsonl");
        let thread = "018f0000-0000-7000-8000-000000000003";
        let events = [
            json!({"v":1,"seq":1,"kind":"genesis","ts":"2026-09-27T00:00:00.000Z",
                "format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws",
                "origin_key":"create","origin_tuple":{"principal":"p","client":"cli",
                    "target":thread,"op":"create","key":"create"},
                "resume":"never","config":{"digest":"cfg"},"identity_profile":"auto"}),
            json!({"v":1,"seq":2,"kind":"input","ts":"2026-09-27T00:00:01.000Z",
                "content":[{"type":"text","text":first}],"origin_key":"input",
                "origin_tuple":{"principal":"p","client":"cli","target":thread,
                    "op":"submit","key":"input"}}),
            json!({"v":1,"seq":3,"kind":"run_start","ts":"2026-09-27T00:00:02.000Z",
                "run":"r1","binary":"worker","config_digest":"cfg",
                "instruction_digest":"ins","policy":"p","mode":"ordinary",
                "recovery_ordinal":0}),
            json!({"v":1,"seq":4,"turn":1,"kind":"turn_open",
                "ts":"2026-09-27T00:00:03.000Z","trigger":{"inputs":[2]}}),
        ];
        let mut bytes = Vec::new();
        for event in events {
            let event = make_event(event).unwrap();
            bytes.extend(event.canonical_bytes().unwrap());
            bytes.push(b'\n');
        }
        std::fs::write(&path, bytes).unwrap();
        (directory, LockedLedger::open(&path, 1).unwrap())
    }

    #[test]
    fn classifies_first_user_request_without_model_work() {
        for request in [
            "Fix the failing test in ledger.rs",
            "Please refactor the API and run tests",
            "修复仓库里的报错，再跑测试",
            "Explain why this Rust function panics",
        ] {
            assert_eq!(
                classify(request),
                tools::IdentityProfile::Coding,
                "{request}"
            );
        }
        for request in [
            "Translate this paragraph into Japanese",
            "Write a short story about a train",
            "总结这篇文章的主要观点",
            "为什么东京的夏天这么热？",
        ] {
            assert_eq!(
                classify(request),
                tools::IdentityProfile::General,
                "{request}"
            );
        }
        assert_eq!(
            classify("Please take care of this"),
            tools::IdentityProfile::Coding
        );
        assert_eq!(classify(""), tools::IdentityProfile::Coding);
    }

    #[test]
    fn selection_is_durable_and_runtime_only() {
        let (_directory, mut ledger) = auto_ledger("翻译这段文字");
        assert_eq!(selected(&ledger).unwrap(), None);
        assert_eq!(
            resolve(&mut ledger, "2026-09-27T00:00:04.000Z", 1).unwrap(),
            tools::IdentityProfile::General
        );
        let count = ledger.projection().unwrap().events.len();
        assert_eq!(
            resolve(&mut ledger, "2026-09-27T00:00:05.000Z", 1).unwrap(),
            tools::IdentityProfile::General
        );
        assert_eq!(ledger.projection().unwrap().events.len(), count);
        let selected_event = ledger.projection().unwrap().events.last().unwrap();
        assert_eq!(
            selected_event.string_field("subkind"),
            Some(SELECTED_SUBKIND)
        );
        assert_eq!(selected_event.effective_visibility(), Visibility::Runtime);
        let initial_system = tools::root_system_instructions(
            selected(&ledger).unwrap().unwrap(),
            "deepseek-flash",
            Some("/work"),
            "",
        );
        let path = ledger.path().to_owned();
        drop(ledger);
        let reopened = LockedLedger::open(&path, 1).unwrap();
        assert_eq!(
            selected(&reopened).unwrap(),
            Some(tools::IdentityProfile::General)
        );
        assert_eq!(
            initial_system,
            tools::root_system_instructions(
                selected(&reopened).unwrap().unwrap(),
                "deepseek-flash",
                Some("/work"),
                "",
            )
        );
    }

    #[test]
    fn explicit_identity_is_stable_without_a_selection_event() {
        let (_directory, mut ledger) = auto_ledger("翻译这段文字");
        // The same prompt must not override an explicit coding profile.
        let path = ledger.path().to_owned();
        drop(ledger);
        let bytes = std::fs::read(&path).unwrap();
        let text = String::from_utf8(bytes).unwrap().replace(
            "\"identity_profile\":\"auto\"",
            "\"identity_profile\":\"coding\"",
        );
        std::fs::write(&path, text).unwrap();
        ledger = LockedLedger::open(&path, 1).unwrap();
        assert_eq!(
            resolve(&mut ledger, "2026-09-27T00:00:04.000Z", 1).unwrap(),
            tools::IdentityProfile::Coding
        );
        assert_eq!(ledger.projection().unwrap().events.len(), 4);
    }
}
