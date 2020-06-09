use dropbox_sdk::{files::upload, HyperClient};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use crate::some_or_continue;
use crate::dropbox_foldering;
pub fn start_uploader(token: String, rx : Receiver<(PathBuf, String)>) {
    std::thread::spawn(move || {
        // Construct this thread own's client for safety
        let client = HyperClient::new(token);
        loop {
            if let Ok((full_path, app_name)) = rx.recv() {
                let file_name = dropbox_foldering::get_file_name(full_path.clone());
                some_or_continue!(&file_name);
                let upload_path = dropbox_foldering::get_upload_path(&file_name.unwrap(), &app_name);
                debug!("Dropbox upload path: {}", &upload_path)
                info!("Uploading app: {}'s file", &app_name)
                match upload(
                    &client,
                    &dropbox_sdk::files::CommitInfo::new(upload_path)
                        .with_mode(dropbox_sdk::files::WriteMode::Overwrite),
                    &std::fs::read(full_path).unwrap(),
                ) {
                    Err(err) => error!("Something went wrong during upload: {}", err),
                    Ok(_) => info!("Succesfully uploaded file")
                };
        } else {
            // TODO: Verify if this case is even possible (I think receiver blocks until new message is received)
                std::thread::sleep(std::time::Duration::from_secs(3));
                continue;
            }
        }
    });
}
