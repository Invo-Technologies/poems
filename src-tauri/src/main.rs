use tauri::command;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let my_variable = env::var("APPNAME").unwrap_or("default_value".to_string());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            decrypt_command,
            register_command,
            env_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn decrypt_command() {
    // Call the decrypt binary
    std::process::Command::new("decrypt")
        .status()
        .expect("Failed to execute decrypt binary");
}

#[tauri::command]
fn register_command() {
    // Call the registration binary
    std::process::Command::new("register")
        .status()
        .expect("Failed to execute registration binary");
}

#[tauri::command]
fn env_command() {
    // Call the environment binary
    std::process::Command::new("env")
        .status()
        .expect("Failed to execute environment binary");
}
