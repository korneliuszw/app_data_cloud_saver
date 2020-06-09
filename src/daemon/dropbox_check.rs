use dropbox_sdk::HyperClient;
use dropbox_sdk::users::{get_space_usage as auth_check};

// There is no echo request for applications marked as App's folder only so instead get space usage (the fastest and the most private)
pub fn is_working(client: &HyperClient) -> bool {
   auth_check(client).is_ok()
}