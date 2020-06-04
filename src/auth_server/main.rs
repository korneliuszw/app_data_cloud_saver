extern crate tokio;
extern crate hyper;

pub static DBX_CLIENT_ID : &'static str= std::env!("DBX_CLIENT_ID", "Missing DBX_CLIENT_ID ");
pub static DBX_CLIENT_SECRET: &'static str = std::env!("DBX_CLIENT_SECRET", "Missing DBX_CLIENT_SECRET");

#[tokio::main]
async fn main () {
    
}