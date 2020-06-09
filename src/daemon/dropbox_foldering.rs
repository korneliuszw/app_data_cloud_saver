static DBX_APP_NAME : &'static str = std::env!("DBX_APP_NAME", "Missing DBX_APP_NAME environment variable");
use dropbox_sdk::HyperClient;
use dropbox_sdk::files::{create_folder, CreateFolderArg};

pub fn get_file_name(mut path: std::path::PathBuf) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}
pub fn get_upload_path(file_name: &String, app_name: &String) -> String {
    format!("/{}/{}", app_name, file_name)
}

// Send folder creation request at given path
// Will log Confllict error if it exists, so don't care about it.
pub fn ensure_folder_existence(folder_path: &String, client: &HyperClient) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Ensuring existence of {}", folder_path);
    create_folder(client, &CreateFolderArg::new(folder_path.clone()))?;
    Ok(())
}