use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

mod components;
mod globals;
mod layout;
mod pages;
mod routes;

use routes::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(index)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
