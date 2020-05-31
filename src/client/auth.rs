use dropbox_sdk::client_trait::HttpClient;
use dropbox_sdk::{HyperClient, Oauth2AuthorizeUrlBuilder, Oauth2Type};
use gtk::prelude::*;
use std::env;
use std::sync::mpsc::channel;

pub fn create_auth_window(builder: &crate::ui_builder::UIBuilder) {
    let auth_window = builder.create_window("window2");
    let link_button: gtk::LinkButton = builder.builder.get_object("oauth_link").unwrap();
    link_button.set_uri(&get_dropbox_oauth_link());
    let code_field: gtk::Entry = builder.builder.get_object("oauth_code").unwrap();
    let code_submit: gtk::Button = builder.builder.get_object("login_submit_button").unwrap();
    code_submit.connect_clicked(move |_| {
        let code = code_field.get_text().unwrap();
        match exchange_code_for_token(code.as_str()) {
            Ok(token) => {
                save_to_file(token).unwrap();
                create_restart_dialog();
                auth_window.close();
            }
            //TODO: Show error message
            _ => {}
        }
    });
}
pub fn get_dropbox_oauth_link() -> String {
    let client_id = &crate::DBX_CLIENT_ID;
    Oauth2AuthorizeUrlBuilder::new(&client_id, Oauth2Type::AuthorizationCode)
        .build()
        .into_string()
}
pub fn exchange_code_for_token(code: &str) -> Result<String, dropbox_sdk::Error> {
    let client_id = &crate::DBX_CLIENT_ID;
    let client_secret = &crate::DBX_CLIENT_SECRET;
    HyperClient::oauth2_token_from_authorization_code(&client_id, &client_secret, code, None)
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
