// Hide the console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// Saves a backup file into the user's Downloads folder and returns where it went.
/// Never overwrites: adds " (2)", " (3)"… if a file with that name already exists.
#[tauri::command]
fn save_backup(app: tauri::AppHandle, file_name: String, contents: String) -> Result<String, String> {
    let safe: String = file_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || "-_.".contains(*c))
        .collect();
    let name = if safe.ends_with(".json") && safe.len() > 5 { safe } else { "gradient-backup.json".to_string() };
    let stem = name.trim_end_matches(".json").to_string();

    let dir = app.path().download_dir().map_err(|e| e.to_string())?;
    let mut path = dir.join(&name);
    let mut i = 2;
    while path.exists() {
        path = dir.join(format!("{stem} ({i}).json"));
        i += 1;
    }
    std::fs::write(&path, contents).map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_backup])
        .setup(|app| {
            let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Gradient")
                .inner_size(1000.0, 680.0)
                .min_inner_size(460.0, 520.0);

            // macOS: keep the real traffic lights, but blend the title bar into the app
            #[cfg(target_os = "macos")]
            let builder = builder
                .title_bar_style(tauri::TitleBarStyle::Overlay)
                .hidden_title(true);

            // Windows / Linux: remove the system title bar; the page draws its own buttons
            #[cfg(not(target_os = "macos"))]
            let builder = builder.decorations(false).shadow(true);

            builder.build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Gradient");
}
