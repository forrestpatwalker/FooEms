mod app_errors;
use std::env;

use app_errors::AppError;

fn main() {
    let mut client_id: String = String::new();
    let mut secret: String = String::new();

    match env::var("plaid_sandbox_client_id"){
        Ok(val) => {
            client_id = val;
        }
        Err(e) => {
            AppError::Credentials(e.to_string());
        }
    }
    match env::var("plaid_sandbox_secret"){
        Ok(val) => {
            secret = val;
        }
        Err(e) => {
            AppError::Credentials(e.to_string());
        }
    }

    
}
