use dropbox_sdk::{HyperClient, Oauth2AuthorizeUrlBuilder, Oauth2Type};
use hyper::{Request, Response, Body, StatusCode};

pub async fn generate_auth_link() -> Response<Body>{
    Response::builder()
        .header("Location", &*crate::DBX_REDIRECT_URI)
        .status(StatusCode::PERMANENT_REDIRECT)
        .body(format!("If you didn't get redirect go to this link: {}", &*crate::DBX_REDIRECT_URI).into())
        .unwrap()
}
pub async fn exchange_code_for_token(req: Request<Body>) -> Response<Body> {
    if let Some(code_value) = req.headers().get("X-Dropbox-Code") {
        let code : &str = code_value.to_str().unwrap_or("");
        if let Ok(token) = HyperClient::oauth2_token_from_authorization_code(&*crate::DBX_CLIENT_ID, &*crate::DBX_CLIENT_SECRET, code, None) {
            return create_simple_response(StatusCode::OK, token)
        } else {
            return create_simple_response(StatusCode::NOT_ACCEPTABLE, "Provided code is not valid!".to_string())
        }
    } else {
        return create_simple_response(StatusCode::BAD_REQUEST, "Missing header value".to_string())
    }
}
fn create_simple_response(status: StatusCode, message: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(message.into())
        .unwrap()
}