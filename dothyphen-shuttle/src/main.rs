use actix_web::{
    error, get, http::header::ContentType, post, web::Json, web::JsonConfig, web::ServiceConfig,
    HttpResponse,
};
use dothyphen::translate;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Serialize, Default)]
struct SamuelResponse {
    output: String,
    error: String,
}

#[derive(Deserialize)]
struct SamuelRequest {
    input: String,
}

#[post("/translate")]
async fn translate_to_morse(request_body: Json<SamuelRequest>) -> HttpResponse {
    let body = SamuelResponse {
        output: translate(&request_body.input).to_string(),
        ..Default::default()
    };

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(body);
}

#[get("/translate")]
async fn translate_to_morse_mna() -> HttpResponse {
    let body = SamuelResponse {
        error: "This endpoint only supports the POST method.".to_string(),
        ..Default::default()
    };

    return HttpResponse::MethodNotAllowed()
        .content_type(ContentType::json())
        .json(body);
}

#[get("/")]
async fn root() -> HttpResponse {
    let body = SamuelResponse {
        error: "Use a POST request to /translation endpoint to perform a translation.".to_string(),
        ..Default::default()
    };

    return HttpResponse::NotFound()
        .content_type(ContentType::json())
        .json(body);
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(translate_to_morse);
        cfg.service(translate_to_morse_mna);
        cfg.service(root);
        cfg.app_data(JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(format!(r#"{{"output":"", "error":"{}"}}"#, err)),
            )
            .into()
        }));
    };
    Ok(config.into())
}
