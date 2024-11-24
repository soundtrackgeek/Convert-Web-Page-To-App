#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
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

fn create_app_directory(domain: &str) -> Result<PathBuf> {
    let app_name = format!("{}-app", domain.replace('.', "-"));
    let output_dir = PathBuf::from("C:/_code/generated-apps").join(&app_name);
    
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }
    fs::create_dir_all(&output_dir)?;
    
    Ok(output_dir)
}

async fn generate_app_files(url: &Url, app_dir: &PathBuf) -> Result<()> {
    let domain = url.domain().unwrap_or("unknown");
    let app_name = domain.replace('.', "-");
    
    // Create src-tauri directory
    let tauri_dir = app_dir.join("src-tauri");
    fs::create_dir_all(&tauri_dir)?;
    
    // Create Cargo.toml
    let cargo_content = format!(
        r#"[package]
name = "{}-app"
version = "0.1.0"
description = "Generated app for {}"
edition = "2021"

[build-dependencies]
tauri-build = {{ version = "1.5.0", features = [] }}

[dependencies]
serde_json = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
tauri = {{ version = "1.5.0", features = ["window-maximize", "window-minimize", "window-close"] }}

[features]
custom-protocol = ["tauri/custom-protocol"]"#,
        app_name, domain
    );
    fs::write(tauri_dir.join("Cargo.toml"), cargo_content)?;
    
    // Create main.rs
    let main_rs_content = format!(
        r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {{
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}}"#
    );
    fs::create_dir_all(tauri_dir.join("src"))?;
    fs::write(tauri_dir.join("src/main.rs"), main_rs_content)?;
    
    // Create build.rs
    let build_rs_content = r#"fn main() {
    tauri_build::build()
}"#;
    fs::write(tauri_dir.join("build.rs"), build_rs_content)?;
    
    // Create tauri.conf.json with the correct structure
    let conf_content = format!(
        r#"{{
  "identifier": "com.{}.app",
  "productName": "{} App",
  "version": "0.1.0",
  "build": {{
    "beforeBuildCommand": "",
    "frontendDist": "../dist"
  }},
  "app": {{
    "windows": [
      {{
        "fullscreen": false,
        "height": 768,
        "width": 1024,
        "resizable": true,
        "title": "{} App",
        "url": "{}",
        "decorations": true,
        "center": true
      }}
    ]
  }},
  "bundle": {{
    "active": true,
    "targets": ["msi"]
  }}
}}"#,
        app_name, domain, domain, url
    );
    fs::write(tauri_dir.join("tauri.conf.json"), conf_content)?;
    
    // Create dist directory (required by Tauri)
    fs::create_dir_all(app_dir.join("dist"))?;
    
    // Create a minimal index.html in dist
    let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Loading...</title>
</head>
<body>
    <div>Loading your app...</div>
</body>
</html>"#;
    fs::write(app_dir.join("dist/index.html"), html_content)?;
    
    // Create build-utils directory and files
    let build_utils_dir = tauri_dir.join("build-utils");
    fs::create_dir_all(&build_utils_dir)?;
    
    // Create a minimal build utils script
    let build_utils_content = r#"// This file is required by Tauri
// It can be empty, but it needs to exist"#;
    fs::write(build_utils_dir.join("main.js"), build_utils_content)?;
    
    Ok(())
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
    
    // Extract domain for app name
    let domain = url.domain()
        .unwrap_or("unknown")
        .to_string();
    
    // Create app directory
    let app_dir = create_app_directory(&domain)
        .map_err(|e| e.to_string())?;
    
    // Generate app files
    generate_app_files(&url, &app_dir)
        .await
        .map_err(|e| e.to_string())?;
    
    // Show preview window
    let label = format!("web-window-{}", window.label());
    let web_window = WindowBuilder::new(
        &app_handle,
        label,
        WindowUrl::External(url.clone())
    )
    .title(format!("{} (Preview)", domain))
    .inner_size(1024.0, 768.0)
    .center()
    .decorations(true)
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;
    
    web_window.show().map_err(|e| e.to_string())?;
    
    // Create build directory and copy schema
    let schema_dir = app_dir.join("src-tauri/schemas");
    fs::create_dir_all(&schema_dir).map_err(|e| e.to_string())?;
    
    // Create a basic schema file
    let schema_content = r#"{
  "$schema": "https://json-schema.org/draft/2019-09/schema#",
  "type": "object",
  "required": ["identifier"],
  "properties": {
    "identifier": { "type": "string" }
  }
}"#;
    fs::write(schema_dir.join("desktop-schema.json"), schema_content)
        .map_err(|e| e.to_string())?;
    
    // Build the standalone app
    let tauri_dir = app_dir.join("src-tauri");
    
    // Build with Tauri
    println!("Building app for {}...", domain);
    let output = Command::new("cargo")
        .current_dir(&tauri_dir)
        .args(["tauri", "build"])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        println!("Build error: {}", error_msg);
        return Err(error_msg.to_string());
    }
    
    let exe_path = tauri_dir
        .join("target/release/bundle/msi")
        .join(format!("{}-app_{}_x64_en-US.msi", domain.replace('.', "-"), "0.1.0"));
    
    Ok(format!("App created successfully! You can find the installer at: {}", exe_path.display()))
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
