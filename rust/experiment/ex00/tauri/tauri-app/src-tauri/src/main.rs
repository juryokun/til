#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn simple_command() {
    println!("I was invoked from JS!");
}
#[tauri::command]
fn command_with_message(message: String) -> String {
    println!("{}", message);
    format!("hello {}", message)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            simple_command,
            command_with_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
