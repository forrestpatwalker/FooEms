mod app_errors;
mod requests;
use std::env;

fn main() {
    let mut client_id: String = env::var("plaid_sandbox_client_id").expect("Failed to retreive the client id from the environment, cannot continue.");
    let mut secret: String = env::var("plaid_sandbox_secret").expect("Failed to retreive the secret from the environment, cannot continue.");


}
