extern crate tokio;
extern crate hyper;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate dropbox_sdk;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate lazy_static;
mod dropbox;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use dropbox_sdk::{Oauth2Type,Oauth2AuthorizeUrlBuilder};


pub static DBX_CLIENT_ID : &'static str= std::env!("DBX_CLIENT_ID", "Missing DBX_CLIENT_ID ");
pub static DBX_CLIENT_SECRET: &'static str = std::env!("DBX_CLIENT_SECRET", "Missing DBX_CLIENT_SECRET");
lazy_static! {
    pub static ref DBX_REDIRECT_URI : String =
        Oauth2AuthorizeUrlBuilder::new(&DBX_CLIENT_ID, Oauth2Type::AuthorizationCode)
            .build()
            .into_string();
}
#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let addr = SocketAddr::new([127,0,0,1].into(), std::env::var("PORT").unwrap_or("3000".to_string()).parse::<u16>().unwrap()).into();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(routing_service))
    });
    let server = Server::bind(&addr).serve(make_svc);
    info!("Server running at port {}", addr.port());
    debug!("Dropbox login url: {}", &*DBX_REDIRECT_URI);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
    Ok(())
}
async fn routing_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Received request");
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/get_dropbox_auth") => Ok(dropbox::generate_auth_link().await),
        (&Method::GET, "/exchange_code") =>  Ok(dropbox::exchange_code_for_token(req).await),
        _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Requested resource not found".into())
                .unwrap()
            )
    }
}