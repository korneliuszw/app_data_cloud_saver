use dropbox_sdk::{HyperClient, Oauth2AuthorizeUrlBuilder, Oauth2Type};
use gtk::prelude::*;
use reqwest::blocking::Client;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
static AUTH_API_URL: &'static str = std::env!("AUTH_API_URL", "Please provide AUTH_API_URL");

pub fn create_auth_window(builder: &crate::ui_builder::UIBuilder) {
    // Get url of auth api at compile time and make it static variable.
    let auth_window = builder.create_window("window2");
    let link_button: gtk::LinkButton = builder.builder.get_object("oauth_link").unwrap();
    let client = Client::new();
    link_button.set_uri(&get_dropbox_oauth_link());
    let code_field: gtk::Entry = builder.builder.get_object("oauth_code").unwrap();
    let code_submit: gtk::Button = builder.builder.get_object("login_submit_button").unwrap();
    let code_wrong: gtk::Label = builder.builder.get_object("wrong_code_label").unwrap();
    code_submit.connect_clicked(move |_| {
        let code = code_field.get_text().unwrap();
        match exchange_code_for_token(code.as_str()) {
            Ok(token) => {
                save_to_file(token).unwrap();
                create_restart_dialog();
                auth_window.close();
            }
            //TODO: Show error message
            Err(was_wrong) => {
                if !was_wrong {
                    return;
                }
                code_wrong.show();
                code_wrong.set_visible(true);
            }
        }
    });
}
pub fn get_dropbox_oauth_link() -> String {
    format!("{}/get_dropbox_auth", AUTH_API_URL)
}
// Err has boolean true if request failed because of wrong code.
pub fn exchange_code_for_token(code: &str) -> Result<String, bool> {
    let verifcation_uri: &str = &format!("{}/exchange_code", AUTH_API_URL);
    let mut header_map = HeaderMap::new();
    header_map.insert("X-Dropbox-Code", HeaderValue::from_str(code).unwrap());
    info!("Sending request");
    let resp = Client::new()
        .get(verifcation_uri)
        .headers(header_map)
        .send()
        .or_else(|_| Err(false))?;
    match resp.status() {
        StatusCode::OK => Ok(resp.text().ok().unwrap()),
        StatusCode::NOT_ACCEPTABLE => Err(true),
        _ => Err(false),
    }
}
pub fn save_to_file(token: String) -> std::io::Result<()> {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("SaveManager");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    config_dir.push("token");
    std::fs::write(config_dir, token)
}
fn create_restart_dialog() {
    let dialog = gtk::MessageDialog::new::<gtk::Window>(
        None,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Info,
        gtk::ButtonsType::Ok,
        "Please restart this program",
    );
    dialog.connect_response(|_, res| {
        if res == gtk::ResponseType::Ok {
            std::process::exit(0);
        }
    });
    dialog.run();
}
