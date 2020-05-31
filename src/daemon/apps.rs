use dashmap::DashMap;
use serde::Deserialize;
use std::path::{Path, PathBuf};
#[derive(Deserialize)]
struct App {
    pub executable: String,
    pub upload_paths: Vec<PathBuf>,
}
impl App {
    pub fn read(file: PathBuf) -> App {
        serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap()
    }
    pub fn into_key_value(self) -> (String, Vec<PathBuf>) {
        let executable_path: PathBuf = self.executable.into();
        (
            executable_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            self.upload_paths,
        )
    }
}
pub fn load_files_into_hashmap(dir: &PathBuf) -> DashMap<String, Vec<PathBuf>> {
    let map = DashMap::new();
    std::fs::read_dir(dir).unwrap().for_each(|file| {
        let app = App::read(file.unwrap().path());
        let (k, v) = app.into_key_value();
        map.insert(k, v);
    });

    map
}
