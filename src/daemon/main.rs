#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate dashmap;
extern crate dirs;
extern crate dropbox_sdk;
extern crate gio;
extern crate gtk;
extern crate notify;
extern crate serde;
extern crate serde_json;
extern crate sysinfo;
extern crate env_logger;
use dashmap::DashMap;
use notify::event::{DataChange, EventKind, ModifyKind, RemoveKind};
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};
use std::sync::mpsc::channel;
mod apps;
mod dropbox_uploader;

lazy_static! {
    static ref PROCESS_UPLOAD_MAP: DashMap<String, std::path::PathBuf> = DashMap::new();
    static ref PREVIOUS_RUN: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn main() -> Result<()> {
    let mut system = sysinfo::System::new_all();
    let mut path = dirs::config_dir().unwrap();
    path.push("SaveManager\\");
    let mut token_path = path.clone();
    token_path.push("token");
    let mut config_path = path.clone();
    config_path.push("apps");
    loop {
        if config_path.exists() && token_path.exists() {
            break;
        }
        std::thread::sleep(Duration::from_secs(15));
    }
    let token = String::from_utf8_lossy(&std::fs::read(token_path).unwrap()).to_string();
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res: Result<notify::Event>| match res {
            Ok(event) => {
                dbg!(&event);
                if event.kind == EventKind::Modify(ModifyKind::Any) {
                    for path in event.paths {
                        apps::App::read(path).to_dashmap();
                    }
                } else if event.kind == EventKind::Remove(RemoveKind::File) {
                    for path in event.paths {
                        apps::find_deleted_app(
                            path.file_name().unwrap().to_str().unwrap().to_string(),
                        );
                    }
                }
            }
            _ => {}
        })
        .unwrap();
    watcher
        .watch(&config_path, RecursiveMode::NonRecursive)
        .unwrap();
    apps::load_files_into_hashmap(&config_path);
    let (tx, rx) = channel();
    dropbox_uploader::start_uploader(token, rx);
    loop {
        // Loop over all processes running
        system.refresh_processes();
        let mut CURRENTLY_RUNNING = vec![];
        dbg!("Process check");
        for (_, proc) in system.get_processes() {
            // Find added processes and insert running into vector
            let proc_name = proc.name().to_string();
            if PROCESS_UPLOAD_MAP.contains_key(&proc_name) {
                CURRENTLY_RUNNING.push(proc_name);
            }
        }
        dbg!(&CURRENTLY_RUNNING);
        // Compare 
        let mut previous_lock = PREVIOUS_RUN.lock().unwrap();
        previous_lock
            .iter()
            .filter(|k| !CURRENTLY_RUNNING.contains(k))
            .for_each(|k| {
                tx.send(PROCESS_UPLOAD_MAP.get(k).unwrap().value().clone()).unwrap();
            });
        *previous_lock = CURRENTLY_RUNNING;

        std::thread::sleep(Duration::from_secs(10));
    }
}
