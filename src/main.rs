mod schema;
mod routes;
mod models;
mod db;
mod utils;

#[macro_use]
extern crate rocket;
extern crate diesel;

use std::path::PathBuf;

use rocket::http::Header;
use rocket::{Request, Response, routes};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::status;

pub struct CORS;

use routes::auth::register::register;
use routes::auth::login::login;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));
    }
}

#[options("/<path..>")]
fn all_options(path: PathBuf) -> status::NoContent {
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)  // CORS Fairing einfügen
        .mount("/", routes![
        all_options,
        register,
        login
    ],
        )
}