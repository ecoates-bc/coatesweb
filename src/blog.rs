use anyhow::Result;
use askama::Template;
use markdown::to_html;
use rocket::{fs::NamedFile, response::content::RawHtml};
use std::{path::Path, fs::read_to_string};

use super::utils::{parse_from_toml, load_toml_doc};

#[derive(Template)]
#[template(path = "blog_link.html")]
struct BlogLink {
    title: String,
    date: String,
    subtitle: String,
    link: String,
    thumbnail: Option<String>,
}

#[derive(Template)]
#[template(path = "blog_year_block.html")]
struct BlogYearBlock {
    year: usize,
    slugs: Vec<String>
}

#[derive(Template)]
#[template(path = "blog_article.html")]
struct BlogArticle {
    title: String,
    subtitle: String,
    date: String,
    content: String,
    thumbnail: Option<String>,
}

#[get("/")]
pub async fn blog_home() -> Option<NamedFile> {
    let index_path = Path::new("static/").join("blog.html");
    NamedFile::open(index_path).await.ok()
}

#[get("/<slug>")]
pub async fn blog_article(slug: String) -> Option<RawHtml<String>> {
    get_blog_article(&slug).ok().and_then(|article| {
        Some(RawHtml(article))
    })
}

fn get_blog_article(slug: &str) -> Result<String> {
    let toml_vals = get_toml_vals(slug)?;
    let content_path = Path::new("static/content/blog")
        .join(slug)
        .join(format!("{slug}.md"));

    let thumbnail_path = format!("/static/content/blog/{slug}/thumbnail.png");
    let thumbnail_buf = Path::new("static/content/blog")
        .join(&slug)
        .join("thumbnail.png");
    let thumbnail_link = if thumbnail_buf.exists() {
        Some(thumbnail_path)
    } else {
        None
    };

    let content = read_to_string(content_path)?;
    let content = to_html(&content);
    let article = BlogArticle {
        title: toml_vals[0].to_owned(),
        date: toml_vals[1].to_owned(),
        subtitle: toml_vals[2].to_owned(),
        content: content,
        thumbnail: thumbnail_link,
    };
    Ok(article.render()?)
}

#[get("/year/<year>")]
pub async fn blog_year(year: usize) -> Option<String> {
    let slugs = match year {
        2023 => vec!["new-website".to_string()],
        _ => vec![]
    };
    let blog_block = BlogYearBlock { year, slugs };
    blog_block.render().ok()
}

#[get("/link/<slug>")]
pub async fn blog_link(slug: String) -> Option<String> {
    get_blog_link(slug).ok()
}

fn get_blog_link(slug: String) -> Result<String> {
    let thumbnail_path = format!("/static/content/blog/{slug}/thumbnail-small.png");
    let thumbnail_buf = Path::new("static/content/blog")
        .join(&slug)
        .join("thumbnail-small.png");
    let thumbnail_link = if thumbnail_buf.exists() {
        Some(thumbnail_path)
    } else {
        None
    };

    let vals = get_toml_vals(&slug)?;
    let blog_link = BlogLink {
        title: vals[0].to_owned(),
        date: vals[1].to_owned(),
        subtitle: vals[2].to_owned(),
        link: vals[3].to_owned(),
        thumbnail: thumbnail_link,
    };

    Ok(blog_link.render()?)
}

fn get_toml_vals(slug: &str) -> Result<Vec<String>> {
    let keys = vec!["title", "date", "subtitle", "link"];
    let toml_path = format!("static/content/blog/{slug}/{slug}.toml");

    let toml = load_toml_doc(&toml_path)?;
    let vals = parse_from_toml(toml, keys)?;

    Ok(vals)
}