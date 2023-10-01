use std::fmt::Display;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::components::Component;

#[derive(Debug)]
pub struct Layout<T>
where
    T: Component + Display,
{
    pub lang: String,
    pub title: String,
    pub root: Option<T>,
}

impl Layout<T> {
    pub fn create<T>(lang: String, title: String, root: Option<T>) -> Self {
        Layout { lang, title, root }
    }
}

impl Component for Layout<T> {
    fn render(&self) -> String {
        let content = match self.root {
            Some(root_component) => root_component.render(),
            None => "",
        };

        let body_html = html!(
                <head>
                    <title>{self.title}</title>
                    <meta charset="UTF-8" />
                    <link href="https://fonts.googleapis.com/css?family=Comfortaa" rel="stylesheet" />
                    <link href="https://fonts.googleapis.com/css?family=Unica+One" rel="stylesheet" />
                    <link href="/static/www/css/style.css" rel="stylesheet" />
                    <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
                </head>
                <body>
                    {content}
                </body>
        );
        let result: String =
            "<!DOCTYPE html><html lang='".to_string() + self.lang.into() + "'>prova</html>".into();
        result
    }
}
