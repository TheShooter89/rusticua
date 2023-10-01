use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::pages::Page;

#[get("/")]
pub async fn index() -> impl Responder {
    let homepage = Page::create("/".to_string(), None);

    let rendered_html = homepage.render();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html)
}
