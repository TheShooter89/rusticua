use actix_files::NamedFile;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};

use html_to_string_macro::html;

use crate::game::{get_random_letters, Letter};

fn app_title_template() -> String {
    let result = html!(
        <head>
            <title>"Cyrillic Alphabet Game"</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link href="/static/www/css/style.css" rel="stylesheet" />
            <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
        </head>
    );

    return result;
}

#[get("/")]
pub async fn app() -> Result<NamedFile> {
    Ok(NamedFile::open("static/www/index.html")?)
}

#[get("/home")]
pub async fn home() -> impl Responder {
    //HttpResponse::Ok().body("START GAME")
    let response_html = html!(

        {app_title_template()}
        <body>"test /home"</body>
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_html)
}

#[get("/main_title")]
pub async fn main_title() -> impl Responder {
    HttpResponse::Ok().body("START GAME")
}

#[get("/random_letters")]
pub async fn random_letters() -> impl Responder {
    let letters = get_random_letters();

    let rendered_html = html!(
        <div>
            <span>{letters[0].character.clone()}</span>
            <span>{letters[0].lowercase.clone()}</span>
            <span>{letters[0].latin_transliteration.clone()}</span>
            <span>{letters[0].italian_pronunciation.clone()}</span>
        </div>
        <div>
            <span>{letters[1].character.clone()}</span>
            <span>{letters[1].lowercase.clone()}</span>
            <span>{letters[1].latin_transliteration.clone()}</span>
            <span>{letters[1].italian_pronunciation.clone()}</span>
        </div>
        <div>
            <span>{letters[2].character.clone()}</span>
            <span>{letters[2].lowercase.clone()}</span>
            <span>{letters[2].latin_transliteration.clone()}</span>
            <span>{letters[2].italian_pronunciation.clone()}</span>
        </div>
    );
    HttpResponse::Ok().body(rendered_html)
}

//#[post("/start_game")]
//pub async fn start_game(request_body: String) -> impl Responder {
//    HttpResponse::Ok().body("START GAME")
//}
