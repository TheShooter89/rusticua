use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::components::Component;

#[derive(Debug)]
pub struct Topbar {
    pub title: String,
    pub icon_path: String,
}

impl Topbar {
    pub fn create(title: String, icon_path: String) -> Self {
        Topbar { title, icon_path }
    }
}

impl Component for Topbar {
    fn render(&self) -> String {
        html!(
            <header>
                <img src={self.icon_path} />
                <h1>
                    {self.title}
                </h1>
            </header>
        )
    }
}
