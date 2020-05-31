#[macro_use]
extern crate lazy_static;
extern crate dashmap;
extern crate dirs;
extern crate gio;
extern crate gtk;
extern crate notify;
extern crate serde;
extern crate serde_json;
extern crate sysinfo;
use notify::event::{DataChange, EventKind, ModifyKind, RemoveKind};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};

mod apps;

lazy_static!()

fn main() {
    let mut system = sysinfo::System::new_all();
    let (tx, rx) = channel();
    let mut path = dirs::config_dir().unwrap();
    path.push("SaveManager/");
    let mut token_path = path.clone();
    token_path.push("token");
    let mut config_path = path.clone();
    config_path.push("apps/");
    loop {
        if config_path.exists() && token_path.exists() {
            break;
        }
        std::thread::sleep(Duration::from_secs(15));
    }
    let token: Arc<String> =
        Arc::new(String::from_utf8_lossy(&std::fs::read(token_path).unwrap()).to_string());
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(4)).unwrap();
    watcher
        .watch(&config_path, RecursiveMode::NonRecursive)
        .unwrap();
    std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(event) => {
                if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Content)) {
                } else if event.kind == EventKind::Remove(RemoveKind::File) {
                }
            }
            _ => {}
        }
    });
    // First we update all information of our system struct.
    system.refresh_all();
    // Now let's print every process' id and name:
    loop {
        for (pid, proc_) in system.get_processes() {}
    }
}
