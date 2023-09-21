use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

mod game;
mod ui;

const INDEX_HTML_PATH: &str = "www/index.html";

struct AppState {
    pub app_name: String,
    pub player: Option<Player>,
}

#[derive(Debug)]
struct Score {
    pub total: i32,
    pub correct: i32,
}

#[derive(Debug)]
struct Player {
    pub name: String,
    pub score: Score,
}

#[get("/")]
async fn index() -> Result<fs::NamedFile, Error> {
    game::get_random_letters();
    let index_file = "static/www/index.html";
    let file = fs::NamedFile::open(index_file).unwrap();
    Ok(file.use_last_modified(true))
}

#[post("/echo")]
async fn echo(request_body: String) -> impl Responder {
    let response_html = html!(<div>"Echo "<strong>"boy"</strong></div>);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_html)
}

#[post("/start_game")]
async fn start_game(request_body: String) -> impl Responder {
    HttpResponse::Ok().body("START GAME")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("MANUAL HELLO acrix world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("cyrillic alphabet game app"),
                player: None,
            }))
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(index)
            .service(start_game)
            //.service(hello)
            .service(echo)
            .service(ui::main_title)
            .service(ui::home)
            .service(ui::random_letters)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
