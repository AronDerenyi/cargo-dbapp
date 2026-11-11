use anyhow::{Context as _, Result};
use cargo_metadata::Metadata;
use clap::builder::styling::{AnsiColor, Style};
use indexmap::IndexMap;
use serde_json::Value;

const WARN: Style = AnsiColor::Yellow.on_default().bold();

pub fn root_package_name(metadata: &Metadata) -> Result<&str> {
    Ok(&metadata
        .root_package()
        .context("run inside a package")?
        .name)
}

pub fn env_vars(metadata: &Metadata) -> IndexMap<&str, String> {
    fn map<'a>((key, value): (&'a String, &Value)) -> Option<(&'a str, String)> {
        Some((
            key,
            match value {
                Value::Null => return None,
                Value::Bool(bool) => bool.to_string(),
                Value::Number(number) => number.to_string(),
                Value::String(string) => string.clone(),
                value => {
                    eprintln!("{WARN}warning{WARN:#}: `{key}` has an invalid value: {value}",);
                    return None;
                }
            },
        ))
    }

    metadata
        .workspace_metadata
        .get("dbapp")
        .and_then(|dbapp| dbapp.get("env"))
        .and_then(|env| env.as_object())
        .map_or_default(|env| env.iter().filter_map(map).collect())
}
