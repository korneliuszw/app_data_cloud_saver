extern crate dirs;
extern crate gio;
extern crate gtk;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sysinfo;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use gio::prelude::*;
use gtk::Application;
use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

// TODO: Refactor this shitty code
mod apps;
mod auth;
mod ui_builder;

fn main() {
    pretty_env_logger::init();
    let application: Application =
        Application::new(Some("com.korneliuszw.save_manager"), Default::default())
            .expect("Failed to create application");
    application.connect_activate(|app| {
        let mut builder: ui_builder::UIBuilder = ui_builder::UIBuilder::obtain_builder(&app);
        if !config_file_exists() {
            info!("Token file does not exists, crying auth window");
            return auth::create_auth_window(&builder);
        } else if !is_daemon_running() {
            // TODO: Show error window
            // start_daemon().unwrap();
        }
        builder.create_main_window();
    });
    debug!("Trying to run application");
    application.run(&[]);
}
fn is_daemon_running() -> bool {
    let mut system: sysinfo::System = sysinfo::System::new_all();
    system.refresh_processes();
    for (_, proc_) in system.get_processes() {
        if proc_.name() == "save_deamon.exe" {
            return true;
        }
    }
    return false;
}
fn start_daemon() -> Result<bool, Box<dyn std::error::Error>> {
    let mut path: std::path::PathBuf = std::env::current_dir()?;
    dbg!(&path);
    path.push("save_daemon.exe");
    Command::new(path.as_path()).spawn()?;
    Ok(true)
}
fn config_file_exists() -> bool {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("SaveManager/token");
    config_dir.exists()
}
