use diva::Command;
use simple_logger::SimpleLogger;
use std::{io::Write as _, path::Path};

// `diva::Error` contains detailed error information; the process failing to
// spawn, the process exiting with a non-zero status, stderr contents, etc.
// For commands with piped output, you'll even have access to the stdout
// contents.
fn main() -> diva::Result<()> {
    // We generate a ton of helpful logging, if you're into that sort of thing.
    SimpleLogger::new().init().unwrap();

    let path = Path::new("src");
    println!("{path:?} directory contents:");
    // By default, commands inherits the parent process's environment. For more
    // reproducability, you can call `pure` afterwards to get a completely
    // empty environment.
    let status = Command::parse("ls -l")
        // `std::process::Command::arg` takes `&mut self` and returns
        // `&mut Self`; our equivalent of that is `add_arg`, but I personally
        // prefer using `with_arg`, which takes `self` and returns `Self`.
        .with_arg(path)
        // We use more explicit names for our run methods than
        // `std::process::Command` does:
        // - `run` (equivalent to `spawn`)
        // - `run_and_wait` (equivalent to `status`)
        // - `run_and_wait_for_output` (equivalent to `output`)
        .run_and_wait()?;
    // `diva::ExitStatus` is just a re-export of `std::process::ExitStatus`.
    println!("exited with code {:?}", status.code());

    let readme_output = Command::parse("cat README.md")
        // Just like with `std::process::Command::output`, this will
        // automatically pipe stdout and stderr.
        .run_and_wait_for_output()?;
    // `diva::Output` has cute conveniences for the very common task of
    // converting output to a string.
    println!(
        "README.md contents:\n{}",
        readme_output
            .stdout_str()
            .expect("README.md contained invalid utf-8")
    );

    let mut handle = Command::new("shasum")
        // We also have methods that let you set these using `diva::Stdio`
        // (which is currently just a re-export of `std::process::Stdio`), but
        // this spares you some typing and an import.
        .with_stdin_piped()
        .with_stdout_piped()
        .with_stderr_piped()
        .run()?;
    handle
        .stdin()
        // This will only be `None` if you forgot to set stdin to piped above.
        .expect("developer error: `handle` stdin not captured")
        .write_all(readme_output.stdout())
        .expect("failed to write to `handle` stdin");
    // `diva::Handle` is very similar to `std::process::Child`, but will
    // log an error message if it's dropped without being waited on.
    let shasum_output = handle.wait_for_output()?;
    println!(
        "README.md SHA-1 sum: {}",
        shasum_output
            .stdout_str()
            .expect("shasum output contained invalid utf-8")
    );

    Ok(())
}
