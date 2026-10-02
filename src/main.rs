mod args;
mod commands;

use crate::{
    args::{Cmd, parse_args},
    commands::{cargo::BuildCommand, databricks::AppsRunLocalCommand},
};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use std::{fs, io::Write};
use tempfile::NamedTempFile;

const TARGET: &str = "x86_64-unknown-linux-musl";
const LINKER: &str = "rust-lld";

const RUN_APP_YML_CONTENT: &str = "command: ['cargo', 'run']\n";
const BUILD_APP_YML_CONTENT: &str = "command: ['sh', 'start.sh']\n";
const START_SH_CONTENT: &str = r#"#!/bin/sh
DIR="$(cd "$(dirname "$0")" && pwd)"
chmod +x "$DIR/bin"
exec "$DIR/bin"
"#;

fn main() -> Result<()> {
    let args = parse_args();

    return match args.cmd {
        Cmd::Run => run(),
        Cmd::Build => build(),
    };
}

fn run() -> Result<()> {
    let mut app_yml_path = NamedTempFile::new()?;
    app_yml_path.write_all(RUN_APP_YML_CONTENT.as_bytes())?;

    AppsRunLocalCommand::new()
        .entry_point(&app_yml_path)
        .exec()?;

    drop(app_yml_path); // Explicit drop to keep the tmp file during execution
    Ok(())
}

fn build() -> Result<()> {
    // Getting metadata
    let metadata = MetadataCommand::new().no_deps().exec()?;
    let target_path = &metadata.target_directory;
    let name = &*metadata
        .root_package()
        .context("run inside a package")?
        .name;

    // Building the binary
    let build_path = target_path.join(TARGET).join("release").join(name);
    BuildCommand::new()
        .release()
        .target_with_linker(TARGET, LINKER)
        .exec()?;

    // Creating the databricks directory
    let databricks_dir = target_path.join("databricks");
    let _ = fs::remove_dir_all(&databricks_dir);
    fs::create_dir_all(&databricks_dir)?;

    // Clopying the built binary to the databricks folder
    fs::copy(build_path, databricks_dir.join("bin"))?;

    // Writing the app.yaml and build.sh
    fs::write(databricks_dir.join("app.yml"), BUILD_APP_YML_CONTENT)?;
    fs::write(databricks_dir.join("start.sh"), START_SH_CONTENT)?;

    Ok(())
}
