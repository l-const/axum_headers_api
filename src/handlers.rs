// Template library
use askama::Template;

use std::convert::Infallible;

// Axum imports
use axum::body::{Bytes, Full};
use axum::http::header::{ACCEPT_LANGUAGE, USER_AGENT};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::Html;
use axum::{response::IntoResponse, Json};

use serde_json::{json, Value};

#[derive(Template)]
#[template(path = "base.html")]
pub(crate) struct WhoAmITemplate {
    ip_address: String,
    language: String,
    software: String,
}

pub(crate) struct HtmlTemplate<T>(T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template, Error: {}",
                    err
                )))
                .unwrap(),
        }
    }
}

pub(crate) async fn text_handler() -> &'static str {
    "Hello axum!\nContent-type: text/plain"
}

pub(crate) async fn template_handler(headers: HeaderMap) -> HtmlTemplate<WhoAmITemplate> {
    let soft = headers.get(USER_AGENT).unwrap();
    let soft_str = std::str::from_utf8(soft.as_bytes()).unwrap();
    let lang = headers.get(ACCEPT_LANGUAGE).unwrap();
    let lang_str = std::str::from_utf8(lang.as_bytes()).unwrap();
    let template = WhoAmITemplate {
        ip_address: "".into(),
        language: lang_str.into(),
        software: soft_str.into(),
    };
    HtmlTemplate(template)
}

pub(crate) async fn json_handler(headers: HeaderMap) -> Json<Value> {
    let soft = headers.get(USER_AGENT).unwrap();
    let soft_str = std::str::from_utf8(soft.as_bytes()).unwrap();
    let lang = headers.get(ACCEPT_LANGUAGE).unwrap();
    let lang_str = std::str::from_utf8(lang.as_bytes()).unwrap();
    let json_str = json!({
        "software" : soft_str,
        "language" : lang_str,
        "ip_address": ""
    });
    Json(json_str)
}
