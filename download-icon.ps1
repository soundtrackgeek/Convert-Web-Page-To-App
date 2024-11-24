# Download a sample icon from icons8
Invoke-WebRequest -Uri "https://img.icons8.com/fluency/96/window.png" -OutFile "icon.png"

# Copy the icon to all required sizes
Copy-Item "icon.png" "src-tauri/icons/32x32.png"
Copy-Item "icon.png" "src-tauri/icons/128x128.png"
Copy-Item "icon.png" "src-tauri/icons/128x128@2x.png"
Copy-Item "icon.png" "src-tauri/icons/icon.ico"
Copy-Item "icon.png" "src-tauri/icons/icon.icns"

# Clean up
Remove-Item "icon.png"
