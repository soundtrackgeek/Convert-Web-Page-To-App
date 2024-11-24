#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConvertRequest {
    url: String,
}

#[tauri::command]
async fn convert_webpage(request: ConvertRequest) -> Result<String, String> {
    println!("Converting webpage: {}", request.url);
    
    // Basic URL validation
    if !request.url.starts_with("http://") && !request.url.starts_with("https://") {
        return Err("Invalid URL: Must start with http:// or https://".to_string());
    }
    
    // TODO: Add more sophisticated webpage conversion logic here
    
    Ok("Webpage conversion started successfully".to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert_webpage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
