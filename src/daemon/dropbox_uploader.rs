use dropbox_sdk::{files::upload, HyperClient};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::Receiver;
pub fn start_uploader(token: String, rx : Receiver<PathBuf>) {
    std::thread::spawn(move || {
        let client = HyperClient::new(token);
        loop {
            if let Ok(path) = rx.recv() {
                upload(
                    &client,
                    &dropbox_sdk::files::CommitInfo::new("".to_string())
                        .with_mode(dropbox_sdk::files::WriteMode::Overwrite),
                    &std::fs::read(path).unwrap(),
                )
                .unwrap();
        } else {
                std::thread::sleep(std::time::Duration::from_secs(3));
                continue;
            }
        }
    });
}
