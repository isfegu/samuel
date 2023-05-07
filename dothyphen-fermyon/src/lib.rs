use anyhow::Result;
use dothyphen::translate;
use http::{Method, StatusCode};
use serde_json::{json, Error, Value};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn handle_dothyphen_fermyon(req: Request) -> Result<Response> {
    if *req.method() != Method::POST {
        return Ok(http::Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Some("".into()))?);
    }

    if req.body().is_none() {
        return Ok(http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Some("".into()))?);
    }

    let body_raw_content = std::str::from_utf8(req.body().as_ref().unwrap());
    if body_raw_content.is_err() {
        return Ok(http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Some("".into()))?);
    }

    let body_values: Result<Value, Error> = serde_json::from_str(body_raw_content.unwrap());
    if body_values.is_err() {
        return Ok(http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Some("".into()))?);
    }

    if body_values.as_ref().unwrap().get("input").is_none() {
        return Ok(http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Some("".into()))?);
    }

    let body_values = body_values.as_ref().unwrap();
    let translated_string = translate(body_values["input"].as_str().unwrap_or_default());
    let response = json!({ "output": translated_string });

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.to_string().into()))?)
}
