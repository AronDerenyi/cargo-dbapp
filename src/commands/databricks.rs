use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use pathdiff::diff_paths;

pub struct AppsRunLocalCommand {
    entry_point: Option<PathBuf>,
}

impl AppsRunLocalCommand {
    pub fn new() -> Self {
        Self { entry_point: None }
    }

    pub fn entry_point(&mut self, entry_point: impl AsRef<Path>) -> &mut Self {
        self.entry_point = Some(entry_point.as_ref().into());
        self
    }

    pub fn exec(&self) -> Result<()> {
        let mut command = Command::new("databricks");
        command.args(["apps", "run-local"]);

        if let Some(entry_point) = &self.entry_point {
            // We must convert to relative path because of a databricks CLI bug
            let entry_point = if entry_point.is_absolute() {
                &diff_paths(entry_point, env::current_dir()?)
                    .context("couldn't find relative path to entry point")?
            } else {
                entry_point
            };

            command.args([
                "--entry-point",
                entry_point
                    .to_str()
                    .context("couldn't read entry point file path")?,
            ]);
        }

        command.status()?;
        Ok(())
    }
}
