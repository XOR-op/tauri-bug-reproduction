// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn create_menu(text: &str) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("id".to_string(), "ID: ".to_string() + text))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("dashboard".to_string(), "Dashboard"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

fn main() {
    let tray_menu = create_menu("Initial");
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "dashboard" => {
                    println!("dashboard");
                    if let Err(err) = app.tray_handle().set_menu(create_menu("Dashboard")) {
                        eprintln!("set menu failed")
                    }
                }
                "quit" => app.exit(0),
                s => {
                    println!("click: {}", s);
                }
            },
            SystemTrayEvent::LeftClick { .. } => {
                println!("left clicked");
                if let Err(err) = app.tray_handle().set_menu(create_menu("Left Click")) {
                    eprintln!("set menu failed")
                }
            }
            _ => {
                println!("other events")
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
