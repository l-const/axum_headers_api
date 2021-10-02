use askama::Template;
use std::convert::Infallible;

use axum::body::{Bytes, Full};
use axum::http::header::{ACCEPT_LANGUAGE, USER_AGENT};
use axum::http::{request, HeaderMap, Response, StatusCode};
use axum::response::Html;
use axum::{handler::get, response::IntoResponse, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(json_handler))
        .route("/api/whoami", get(template_handler));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    dbg!(addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}

async fn handler() -> &'static str {
    "Hello axum!"
}

#[derive(Template)]
#[template(path = "base.html")]
struct WhoAmITemplate {
    ip_address: String,
    language: String,
    software: String,
}

struct HtmlTemplate<T>(T);

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
async fn template_handler(headers: HeaderMap) -> impl IntoResponse {
    let soft = headers.get(USER_AGENT).unwrap();
    let soft_str = std::str::from_utf8(soft.as_bytes()).unwrap();
    let lang = headers.get(ACCEPT_LANGUAGE).unwrap();
    let lang_str = std::str::from_utf8(soft.as_bytes()).unwrap();
    let template = WhoAmITemplate {
        ip_address: "".into(),
        language: lang_str.into(),
        software: soft_str.into(),
    };
    HtmlTemplate(template)
}

async fn json_handler() -> impl IntoResponse {
    unimplemented!()
}
