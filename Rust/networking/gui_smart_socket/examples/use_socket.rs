#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use gui_smart_socket::{SmartSocketClient, LOCAL_IP};

use eframe::egui;

#[derive(Debug)]
enum AppError {
    EframeError(eframe::Error),
    ConnectedError(anyhow::Error),
}

fn main() -> Result<(), AppError> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let client = SmartSocketClient::new(LOCAL_IP).map_err(AppError::ConnectedError)?;

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: false,
        initial_window_pos: None,
        initial_window_size: Some(egui::vec2(350.0, 650.0)),
        min_window_size: Some(egui::vec2(350.0, 650.0)),
        max_window_size: Some(egui::vec2(350.0, 650.0)),
        resizable: false,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        ..Default::default()
    };

    eframe::run_native(
        "Application",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(client)
        }),
    )
    .map_err(AppError::EframeError)
}
