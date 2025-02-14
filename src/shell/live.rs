use anyhow::{Result, Context, bail};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};


/// # Description
/// Executes a command
///
/// Sources /usr/share/2/bin/e-core
///
/// Prints each line unless quiet is passed
///
/// **Fail conditions:**
/// - command failed
/// - bash wasn't found
/// - failed to source /usr/share/2/bin/e-core
/// - some sync shenanigans (unlikely)
/// - failing to read stderr/stdout (unlikely)
pub fn exec(command: &str) -> Result<()> {
    // initialize the bash environment
    let command = format!(
    r"
    source /usr/share/2/bin/e-core || exit 211
    {command}
    "
    );

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn bash")?;

    let stdout = child.stdout.take().context("Stdout already taken?")?;
    let stderr = child.stderr.take().context("Stderr already taken?")?;

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line.unwrap();

            println!("\x1b[30;3m{line}\x1b[0m");
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line.unwrap();

            println!("\x1b[31;1m{line}\x1b[0m");
        }
    });

    let status = child.wait()?;
    if !status.success() {
        bail!("Command failed");
    }

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}
