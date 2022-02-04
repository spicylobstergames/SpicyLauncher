#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
async fn get_versions() -> Result<Vec<String>, ()> {
    Ok(vec![String::from("v0.3"), String::from("v0.4")])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
