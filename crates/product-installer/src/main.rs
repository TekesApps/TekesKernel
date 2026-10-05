use std::io::Write;
use tekes_kernel_installer::{Arguments, CONTRACT, Failure};
fn main() {
    let result = (|| {
        let args = std::env::args_os()
            .skip(1)
            .map(|s| s.into_string().map_err(|_| Failure("invalid-arguments")))
            .collect::<Result<Vec<_>, _>>()?;
        match Arguments::parse(&args)? {
            None => std::io::stdout().write_all(CONTRACT)?,
            Some(args) => {
                let reply = tekes_kernel_installer::execute(args)?;
                serde_json::to_writer(std::io::stdout(), &reply)?;
                std::io::stdout().write_all(b"\n")?;
            }
        }
        Ok::<_, Failure>(())
    })();
    if let Err(error) = result {
        let _ = std::io::stderr().write_all(&tekes_kernel_installer::error_reply(error));
        std::process::exit(65);
    }
}
