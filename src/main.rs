mod args;
mod cargo;

use crate::{
    args::{Cmd, parse_args},
    cargo::BuildCommand,
};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use std::fs;

const TARGET: &str = "x86_64-unknown-linux-musl";
const LINKER: &str = "rust-lld";

const APP_YAML_CONTENT: &str = "command: ['sh', 'start.sh']\n";
const START_SH_CONTENT: &str = r#"#!/bin/sh
DIR="$(cd "$(dirname "$0")" && pwd)"
chmod +x "$BIN/bin"
exec "$BIN/bin"
"#;

fn main() -> Result<()> {
    let args = parse_args();

    return match args.cmd {
        Cmd::Build => build(),
    };
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
    fs::remove_dir_all(&databricks_dir)?;
    fs::create_dir_all(&databricks_dir)?;

    // Clopying the built binary to the databricks folder
    fs::copy(build_path, databricks_dir.join("bin"))?;

    // Writing the app.yaml and build.sh
    fs::write(databricks_dir.join("app.yaml"), APP_YAML_CONTENT)?;
    fs::write(databricks_dir.join("start.sh"), START_SH_CONTENT)?;

    return Ok(());
}
