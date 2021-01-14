extern crate wapc_guest as guest;

use guest::prelude::*;
use serde::Serialize;

#[no_mangle]
pub extern "C" fn wapc_init() {
    register_function("validate", validate);
}

#[derive(Serialize, Debug)]
pub(crate) struct ValidationResponse {
    pub accepted: bool,
    pub message: Option<String>,
    pub code: Option<u16>,
}

fn validate(req: &[u8]) -> CallResult {
    let req_obj: serde_json::Value = serde_json::from_slice(req)?;

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

    Ok(serde_json::to_vec(&resp)?)
}
