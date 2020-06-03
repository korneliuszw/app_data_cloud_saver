use serde::{Deserialize, Serialize};

pub struct Apps(pub Vec<App>);
#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub executable: String,
    // TODO: Allow for multiple files
    pub upload_path: String,
}
impl Apps {
    pub fn read() -> std::io::Result<Apps> {
        let mut apps_path = dirs::config_dir().unwrap();
        apps_path.push("SaveManager\\apps");
        if !apps_path.exists() {
            std::fs::create_dir(apps_path).unwrap();
            return Ok(Apps(Vec::new()));
        }
        let apps: Vec<App> = std::fs::read_dir(apps_path)?
            .map(|file| {
                dbg!(&file);
                serde_json::from_slice(&std::fs::read(file.unwrap().path()).unwrap()).unwrap()
            })
            .collect();
        dbg!(&apps);
        Ok(Apps(apps))
    }
}
impl App {
    pub fn save(&mut self) -> std::io::Result<()> {
        std::fs::write(self.get_path(), serde_json::to_string(&self).unwrap())
    }
    pub fn delete(&self) -> std::io::Result<()> {
        std::fs::remove_file(self.get_path())
    }
    fn get_path(&self) -> std::path::PathBuf {
        let mut path = dirs::config_dir().unwrap();
        path.push("SaveManager\\apps");
        let file_name = get_filename(&self.name);
        path.push(file_name);
        path
    }
}
fn get_filename(name: &String) -> String {
    name.replace(" ", "_").to_string()
}
