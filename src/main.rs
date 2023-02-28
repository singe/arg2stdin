use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "usage: {} <commands> <stdin>", args[0])?;
        std::process::exit(1);
    }
    let cmd = &args[1];
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(args[2].as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    io::stdout()
        .write_all(&output.stdout)
        .expect("Failed to write to stdout");
    Ok(())
}
