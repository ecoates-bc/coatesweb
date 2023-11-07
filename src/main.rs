use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;

pub mod publications;
use publications::{pub_link, pub_year, pubs_main_page};

pub mod home;
use home::{home_content, index};

pub mod blog;
use blog::{blog_link, blog_year, blog_article, blog_home};

pub mod utils;

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/notfound.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, home_content, favicon])
        .mount("/blog", routes![blog_link, blog_year, blog_article, blog_home])
        .mount("/publications", routes![pubs_main_page, pub_year, pub_link])
        .mount("/static", routes![files])
        .register("/", catchers![not_found])
}
