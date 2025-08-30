use std::{
    env,
    io::{self, Write},
    path,
};

pub fn print() {
    let cwd = env::current_dir().unwrap_or_else(|_| "-".into());
    let mut display_path = cwd.to_string_lossy().into_owned();

    if let Some(home) = crate::dirs::home() {
        if cwd.starts_with(&home) {
            let stripped = cwd.strip_prefix(&home).unwrap_or(&cwd);
            if stripped.as_os_str().is_empty() {
                display_path = "~".to_string();
            } else {
                display_path = format!("~{}{}", path::MAIN_SEPARATOR, stripped.to_string_lossy());
            }
        }
    }

    print!("{}> ", display_path);
    io::stdout().flush().unwrap();
}
