use crate::{Failure, Result, require};
use std::{
    fs,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
pub struct Output {
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
impl Output {
    pub fn success(&self) -> bool {
        self.status == 0 && self.stderr.is_empty()
    }
}
pub fn run(exe: impl AsRef<Path>, args: &[&str], seconds: u64) -> Result<Output> {
    let dir = tempfile::Builder::new()
        .prefix("tekes-kernel-product-child-")
        .tempdir()?;
    let out = dir.path().join("stdout");
    let err = dir.path().join("stderr");
    let open = |p: &Path| {
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(p)
    };
    let mut child = Command::new(exe.as_ref())
        .args(args)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(open(&out)?)
        .stderr(open(&err)?)
        .spawn()
        .map_err(|_| Failure("operation-launch-failed"))?;
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break s;
        }
        if Instant::now() >= deadline {
            unsafe {
                libc::kill(child.id() as i32, libc::SIGTERM);
            }
            let grace = Instant::now() + Duration::from_secs(2);
            while Instant::now() < grace && child.try_wait()?.is_none() {
                thread::sleep(Duration::from_millis(20));
            }
            let _ = child.kill();
            let _ = child.wait();
            return Err(Failure("operation-timeout"));
        }
        thread::sleep(Duration::from_millis(20));
    };
    require(
        fs::metadata(&out)?.len() <= 65536 && fs::metadata(&err)?.len() <= 65536,
        "operation-output-too-large",
    )?;
    Ok(Output {
        status: status.code().unwrap_or(-1),
        stdout: fs::read(out)?,
        stderr: fs::read(err)?,
    })
}
