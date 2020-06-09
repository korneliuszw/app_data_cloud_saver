#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate dashmap;
extern crate dirs;
extern crate dropbox_sdk;
extern crate notify;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;
extern crate sysinfo;
use dashmap::DashMap;
use notify::event::{EventKind, ModifyKind, RemoveKind};
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};
mod apps;
mod dropbox_uploader;
mod dropbox_check;
mod dropbox_foldering;
mod helpers;
lazy_static! {
    // Hold two static variables
    // PROCESS_UPLOAD_MAP is DashMap (better HashMap) which holds all applications, with executable as key (for easier executable matching)
    // Second ones holds previously ran executables so we can check which are down.
    static ref PROCESS_UPLOAD_MAP: DashMap<String, (std::path::PathBuf, String) > = DashMap::new();
    static ref PREVIOUS_RUN: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut system = sysinfo::System::new_all();
    let mut path = dirs::config_dir().unwrap();
    path.push("SaveManager\\");
    let mut token_path = path.clone();
    token_path.push("token");
    let mut config_path = path.clone();
    config_path.push("apps");
    // Wait for configs (apps) folder and token file existance
    loop {
        if config_path.exists() && token_path.exists() {
            break;
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    // Read token and start cleint
    let token = String::from_utf8_lossy(&std::fs::read(token_path).unwrap()).to_string();
    let client = dropbox_sdk::HyperClient::new(token.clone());
    if !dropbox_check::is_working(&client) {
        error!("Client failed, either because of connection error or auth");
        // TODO: Better way to notify user about error
    }
    apps::load_files_into_hashmap(&config_path, &client);
    // Add file watcher to apps folder and do something on some changes
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res: Result<notify::Event>| match res {
            Ok(event) => {
                // On file's content modify read it into app and insert into dashmap if not exists
                if event.kind == EventKind::Modify(ModifyKind::Any) {
                    info!("File modification detected");
                    for path in event.paths {
                        let app = apps::App::read(path);
                        app.create_folder_in_dropbox(&client);
                        app.to_dashmap();
                    }
                }
                // On file's removal remove it from dashmap
                else if event.kind == EventKind::Remove(RemoveKind::File) {
                    info!("File removal detected");
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
    let (tx, rx) = channel();
    dropbox_uploader::start_uploader(token, rx);
    loop {
        // Loop over all processes running
        system.refresh_processes();
        let mut CURRENTLY_RUNNING = vec![];
        info!("Process check");
        for (_, proc) in system.get_processes() {
            // Find added processes and insert running into vector
            let proc_name = proc.name().to_string();
            if PROCESS_UPLOAD_MAP.contains_key(&proc_name) {
                CURRENTLY_RUNNING.push(proc_name);
            }
        }
        debug!("Running processes: {}", &CURRENTLY_RUNNING);
        // Comparasion
        let mut previous_lock = PREVIOUS_RUN.lock().unwrap();
        previous_lock
            .iter()
            .filter(|k| !CURRENTLY_RUNNING.contains(k))
            .for_each(|k| {
                // Send closed executable's info to uploading queue
                tx.send(PROCESS_UPLOAD_MAP.get(k).unwrap().value().clone()).unwrap();
            });
        *previous_lock = CURRENTLY_RUNNING;

        std::thread::sleep(Duration::from_secs(5));
    }
}
