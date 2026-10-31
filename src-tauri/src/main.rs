// Hide the console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
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
