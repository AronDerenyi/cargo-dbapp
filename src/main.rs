mod args;
mod commands;
mod metadata;
mod yaml;

use crate::{
    args::{Cmd, parse_args},
    commands::{cargo::BuildCommand, databricks::AppsRunLocalCommand},
    metadata::{env_vars, root_package_name},
    yaml::{AppExecutionType, app_yaml},
};
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use std::{fs, io::Write};
use tempfile::NamedTempFile;

const TARGET: &str = "x86_64-unknown-linux-musl";
const LINKER: &str = "rust-lld";

fn main() -> Result<()> {
    let args = parse_args();

    return match args.cmd {
        Cmd::Run => run(),
        Cmd::Build => build(),
    };
}

fn run() -> Result<()> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    let env_vars = env_vars(&metadata);

    let mut app_yml_path = NamedTempFile::new()?;
    app_yml_path.write_all(app_yaml(AppExecutionType::LOCAL, env_vars)?.as_bytes())?;

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
    let name = root_package_name(&metadata)?;
    let env_vars = env_vars(&metadata);

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
    fs::copy(build_path, databricks_dir.join(name))?;

    // Writing the app.yaml and build.sh
    fs::write(
        databricks_dir.join("app.yml"),
        app_yaml(
            AppExecutionType::SHELL {
                binary: name.into(),
            },
            env_vars,
        )?,
    )?;

    Ok(())
}
