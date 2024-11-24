#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use serde::Deserialize;
use tauri::{Manager, WindowBuilder, WindowUrl};
use url::Url;

#[derive(Debug, Deserialize)]
struct ConvertRequest {
    url: String,
}

async fn validate_url(url_str: &str) -> Result<Url> {
    let url = Url::parse(url_str)
        .with_context(|| format!("Failed to parse URL: {}", url_str))?;
    
    if url.scheme() != "http" && url.scheme() != "https" {
        anyhow::bail!("URL must use http or https protocol");
    }
    
    Ok(url)
}

#[tauri::command]
async fn convert_webpage(
    window: tauri::Window,
    app_handle: tauri::AppHandle,
    request: ConvertRequest
) -> Result<String, String> {
    println!("Converting webpage: {}", request.url);
    
    // Validate URL
    let url = validate_url(&request.url)
        .await
        .map_err(|e| e.to_string())?;
    
    // Create a new window with the webpage
    let label = format!("web-window-{}", window.label());
    let web_window = WindowBuilder::new(
        &app_handle,
        label,
        WindowUrl::External(url)
    )
    .title(format!("Converted App - {}", request.url))
    .inner_size(800.0, 600.0)
    .build()
    .map_err(|e| e.to_string())?;
    
    Ok("Webpage converted to app successfully".to_string())
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
