use std::fmt::Display;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::components::{Component, Topbar};
use crate::layout::Layout;

#[derive(Debug)]
pub struct Page<T>
where
    T: Component + Display,
{
    pub uri: String,
    pub root: Option<T>,
}

impl<T> Page<T>
where
    T: Component + Display,
{
    pub fn create(uri: String, root: Option<T>) -> Self {
        Page { uri, root }
    }
}

impl<T> Component for Page<T>
where
    T: Component + Display,
{
    fn render(&self) -> String {
        let topbar = Topbar::create(
            "rusticUA".to_string(),
            "/static/www/images/ua_trident.png".to_string(),
        );

        let layout = Layout::create("en".to_string(), "rusticua - app".to_string(), Some(topbar));

        match self.root {
            Some(page_root) => layout.render(),
            None => html!(""),
        }
    }
}
