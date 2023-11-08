use std::str::FromStr;

use reqwest::{Method, Url, blocking::{self}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrustRequest {
    method: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrustResponse {
    status: u8,
    body: String,
}

impl CrustRequest {
    pub fn new(method: String, uri: String) -> Self {
        CrustRequest {
            method: method,
            uri: uri,
        }
    }

    pub fn execute_request(&self) -> blocking::Response {
        let req = reqwest::blocking::Request::new(
            Method::from_str(&self.method.as_str()).unwrap(),
            Url::from_str(self.uri.as_str()).unwrap()
        );
        
        let client = blocking::Client::new();
        blocking::Client::execute(&client, req).unwrap()
    }
}