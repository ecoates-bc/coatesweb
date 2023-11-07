use anyhow::Result;
use askama::Template;
use markdown::to_html;
use rocket::fs::NamedFile;
use std::path::Path;

use super::utils::{parse_from_toml, load_toml_doc};

#[derive(Template)]
#[template(path = "pub_link.html")]
struct PubLink {
    title: String,
    authors: String,
    venue: String,
    link: String,
}

#[derive(Template)]
#[template(path = "pub_year_block.html")]
struct PubYearBlock {
    year: usize,
    slugs: Vec<String>,
}

#[get("/")]
pub async fn pubs_main_page() -> Option<NamedFile> {
    let index_path = Path::new("static/").join("publications.html");
    NamedFile::open(index_path).await.ok()
}

#[get("/year/<year>")]
pub async fn pub_year(year: usize) -> Option<String> {
    let slugs = match year {
        2023 => vec!["coates2023".to_string()],
        2022 => vec![
            "forbes-et-al2022".to_string(),
            "oliver-et-al2022".to_string(),
        ],
        2021 => vec!["nicolai-et-al2021".to_string()],
        _ => vec![],
    };
    let pub_block = PubYearBlock { year, slugs };
    pub_block.render().ok()
}

#[get("/link/<slug>")]
pub async fn pub_link(slug: String) -> Option<String> {
    get_pub_link(slug).ok()
}

fn get_pub_link(slug: String) -> Result<String> {
    let pub_path = format!("static/content/publications/{slug}.toml");
    let keys = vec!["title", "authors", "venue", "link"];

    let toml = load_toml_doc(&pub_path)?;
    let vals = parse_from_toml(toml, keys)?;
    let pub_link = PubLink {
        title: vals[0].to_owned(),
        authors: to_html(&vals[1]),
        venue: vals[2].to_owned(),
        link: vals[3].to_owned(),
    };

    Ok(pub_link.render()?)
}