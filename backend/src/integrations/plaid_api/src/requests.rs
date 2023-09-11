#[derive(Debug, PartialEq)]
pub struct PlaidClient {
    pub domain: String,
    pub path: String,
    pub url: String,
    pub id: String,
    pub secret: String,
    pub key: Option<String>,
}

impl PlaidClient {
    fn new(id: &str, secret: &str) -> Self {
        let domain = "https://plaid.com/".to_string();
        let path = "transactions/get".to_string();
        let url = format!("{domain}{path}");

        todo!();  
        // TODO
        // Implement a connection to the server and check if
        // the key exists. If it does grab the value and set the key
        // in PlaidClient. If not call Plaid API to create the key and store it.
    }
}

