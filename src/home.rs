use askama::Template;
use markdown::to_html;
use rocket::{fs::NamedFile, Route};
use std::{fs::read_to_string, path::Path};

#[derive(Template, Debug)]
#[template(path = "home.html")]
struct Home {
    content: String,
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let index_path = Path::new("static/").join("index.html");
    println!("{:?}", std::env::current_dir());
    NamedFile::open(index_path).await.ok()
}

#[get("/home-content")]
pub async fn home_content() -> Option<String> {
    let content_path = Path::new("static/content/home.md");
    let template = read_to_string(content_path)
        .and_then(|md| Ok(to_html(&md)))
        .and_then(|content| Ok(Home { content }))
        .ok();

    if let Some(home) = template {
        home.render().ok()
    } else {
        None
    }
}

pub fn home_routes() -> Vec<Route> {
    routes![index, home_content]
}
