use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Apps {
    pub apps: Vec<App>,
}
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
        apps_path.push("SaveManager/apps");
        if !apps_path.exists() {
            std::fs::create_dir(apps_path);
            return Ok(Apps { apps: Vec::new() });
        }
        let apps: Vec<App> = std::fs::read_dir(apps_path)?
            .map(|file| {
                serde_json::from_slice(&std::fs::read(file.unwrap().path()).unwrap()).unwrap()
            })
            .collect();
        Ok(Apps { apps })
    }
}
impl App {
    pub fn save(&self) -> std::io::Result<()> {
        let mut path = dirs::config_dir().unwrap();
        path.push("SaveManager/apps");
        let mut file_name = self.name.replace(" ", "_").escape_unicode().to_string();
        file_name.make_ascii_lowercase();
        path.push(file_name);
        std::fs::write(path, serde_json::to_string(&self).unwrap())
    }
}
