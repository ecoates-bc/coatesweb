use std::{fs::read_to_string, path::Path};
use anyhow::{Result, bail};
use toml_edit::Document;

pub fn parse_from_toml(doc: Document, keys: Vec<&str>) -> Result<Vec<String>> {
    let parsed: Option<Vec<&str>> = keys
        .iter()
        .map(|key| doc.get(key).and_then(|v| v.as_str()))
        .collect();

    match parsed {
        Some(vals) => Ok(
            vals.iter().map(|v| {
                v.to_string()
            }).collect()
        ),
        None => bail!("Unable to parse toml document"),
    }
}

pub fn load_toml_doc(path: &str) -> Result<Document> {
    let as_path = Path::new(path);
    let toml_content = read_to_string(as_path)?;
    let doc = toml_content.parse::<Document>();
    Ok(doc?)
}