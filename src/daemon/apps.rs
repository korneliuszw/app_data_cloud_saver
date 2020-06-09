use serde::Deserialize;
use std::path::{PathBuf};
use crate::dropbox_foldering;
#[derive(Deserialize)]
pub struct App {
    pub executable: String,
    pub upload_path: PathBuf,
    pub name : String,
}
impl App {
    /// Read application's config file into class
    pub fn read(file: PathBuf) -> App {
        serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap()
    }
    /// Send folder creation request to dropbox (if it doesn't exists) based on upload_path
    pub fn create_folder_in_dropbox(&self, client: &dropbox_sdk::HyperClient) -> Result<(), Box<dyn std::error::Error>> {
        dropbox_foldering::ensure_folder_existence(&format!("/{}", &self.name), client)?;
        Ok(())
    }
    /// Constume this struct and create tuple with filename and path to upload
    fn into_key_value(self) -> (String, (PathBuf, String)) {
        let executable_path: PathBuf = self.executable.into();
        (
            executable_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .replace("%20", " "),
            (self.upload_path, self.name)
        )
    }
    /// 
    pub fn to_dashmap(self) {
        let (key, value) = self.into_key_value();
        if crate::PROCESS_UPLOAD_MAP.contains_key(&key) {
            crate::PROCESS_UPLOAD_MAP.replace(key, value);
            return ();
        }
        crate::PROCESS_UPLOAD_MAP.insert(key, value);
    }
}
pub fn load_files_into_hashmap(dir: &PathBuf, client: &dropbox_sdk::HyperClient) {
    std::fs::read_dir(dir).unwrap().for_each(|file| {
        let app = App::read(file.unwrap().path());
        app.create_folder_in_dropbox(client);
        let (k, v) = app.into_key_value();
        crate::PROCESS_UPLOAD_MAP.insert(k, v);
    });
}
pub fn find_deleted_app(app_file: String) {
    let real_name = app_file.replace("_", " ");
    crate::PROCESS_UPLOAD_MAP.remove(&real_name);
}