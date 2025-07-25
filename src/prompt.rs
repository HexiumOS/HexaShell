use std::{
    io::{self, Write},
    path,
};

pub fn print() {
    let cwd = std::env::current_dir().unwrap_or_else(|_| "-".into());

    let display_path = if let Some(home) = crate::dirs::home() {
        if cwd.starts_with(&home) {
            let stripped = cwd.strip_prefix(&home).unwrap_or(&cwd);
            if stripped.as_os_str().is_empty() {
                "~".to_string()
            } else {
                format!(
                    "~{}",
                    path::MAIN_SEPARATOR.to_string() + &stripped.to_string_lossy()
                )
            }
        } else {
            cwd.to_string_lossy().into_owned()
        }
    } else {
        cwd.to_string_lossy().into_owned()
    };

    print!("{}> ", display_path);
    io::stdout().flush().unwrap();
}
