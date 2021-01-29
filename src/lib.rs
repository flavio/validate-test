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

fn validate(validation_req: &[u8]) -> CallResult {
    let validation_req_obj: serde_json::Value = serde_json::from_slice(validation_req)?;
    let settings = validation_req_obj.get("settings");

    let resp: ValidationResponse;

    if settings.is_none() {
        resp = ValidationResponse {
            accepted: false,
            message: Some(String::from("No settings provided to the policy")),
            code: Some(500),
        };
        return Ok(serde_json::to_vec(&resp)?);
    }
    let settings = settings.unwrap();

    let valid_namespace = settings.get("valid_namespace");
    if valid_namespace.is_none() {
        resp = ValidationResponse {
            accepted: false,
            message: Some(String::from(
                "Warning: 'valid_namespace' parameter missing from policy settings",
            )),
            code: None,
        };
        return Ok(serde_json::to_vec(&resp)?);
    }

    let valid_namespace = valid_namespace.unwrap().as_str().unwrap();
    let req_obj = validation_req_obj.get("request").unwrap();

    resp = match req_obj.get("namespace") {
        None => ValidationResponse {
            accepted: false,
            message: Some(String::from("namespace missing from admission request")),
            code: Some(400),
        },
        Some(namespace) => ValidationResponse {
            accepted: (namespace.as_str().unwrap() == valid_namespace),
            message: None,
            code: None,
        },
    };

    Ok(serde_json::to_vec(&resp)?)
}
