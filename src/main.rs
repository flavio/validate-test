use serde::Serialize;

use std::io::{self, Read};

#[derive(Serialize, Debug)]
pub(crate) struct ValidationResponse {
    pub accepted: bool,
    pub message: Option<String>,
    pub code: Option<u16>,
}

fn main() -> std::result::Result<(), std::io::Error> {
    let mut req = String::new();
    io::stdin().read_to_string(&mut req)?;

    let req_obj: serde_json::Value = serde_json::from_str(&req)?;

    let resp: ValidationResponse = match req_obj.get("namespace") {
        None => ValidationResponse {
            accepted: false,
            message: Some(String::from("namespace missing from admission request")),
            code: Some(400),
        },
        Some(namespace) => ValidationResponse {
            accepted: (namespace.as_str().unwrap() == "valid"),
            message: None,
            code: None,
        },
    };

    println!("{}", serde_json::to_string(&resp)?);
    Ok(())
}
