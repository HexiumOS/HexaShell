use std::{
    io::{self},
    process::{Command, Stdio, exit},
};

pub mod dirs;
pub mod prompt;

fn main() {
    loop {
        prompt::print();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read user input");
            continue;
        }

        if input.trim().is_empty() {
            continue;
        }

        if input.trim() == "exit" {
            exit(0);
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        let mut child = match Command::new(command)
            .args(&args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Error executing {}: {}", command, e);
                continue;
            }
        };

        if let Err(e) = child.wait() {
            eprintln!("Failed to work on child process: {}", e);
        }
    }
}
