use std::{
    env,
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

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        if command == "exit" {
            exit(0);
        }

        if command == "cd" {
            let new_dir: String = if let Some(dir) = args.get(0) {
                dir.to_string()
            } else {
                env::var("HOME").unwrap_or_else(|_| "/".to_string())
            };
            if let Err(e) = env::set_current_dir(&new_dir) {
                eprintln!("cd: {}: {}", new_dir, e);
            }
            continue;
        }

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
