use dropbox_sdk::{files::upload, HyperClient};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
pub fn add_to_queue(k: &String, tx: &Sender<PathBuf>) {
    dbg!("Adding to queue");
    if let Some(obj) = crate::PROCESS_UPLOAD_MAP.get(k) {
        tx.send(obj.value().clone()).unwrap();
    };
}
pub fn start_uploader(token: String, rx: Receiver<PathBuf>) {
    let receiver_save = Arc::new(Mutex::new(rx));
    std::thread::spawn(move || {
        let client = HyperClient::new(token);
        loop {
            let first_at_queue = receiver_save.lock().unwrap().recv();
            if first_at_queue.is_err() {
                std::thread::sleep(std::time::Duration::from_secs(3));
                continue;
            }
            let path = first_at_queue.unwrap();
            dbg!(&path);
            let upload_path = format!("{}", path.file_name().unwrap().to_str().unwrap());
            upload(
                &client,
                &dropbox_sdk::files::CommitInfo::new(upload_path)
                    .with_mode(dropbox_sdk::files::WriteMode::Overwrite),
                &std::fs::read(path).unwrap(),
            )
            .unwrap();
        }
    });
}
