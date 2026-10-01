use std::process::Command;

use anyhow::Result;

pub struct BuildCommand {
    release: bool,
    target: Option<(String, Option<String>)>,
}

impl BuildCommand {
    pub fn new() -> Self {
        BuildCommand {
            release: false,
            target: None,
        }
    }

    pub fn release(&mut self) -> &mut Self {
        self.release = true;
        self
    }

    pub fn target(&mut self, target: impl AsRef<str>) -> &mut Self {
        self.target = Some((target.as_ref().into(), None));
        self
    }

    pub fn target_with_linker(
        &mut self,
        target: impl AsRef<str>,
        linker: impl AsRef<str>,
    ) -> &mut Self {
        self.target = Some((target.as_ref().into(), Some(linker.as_ref().into())));
        self
    }

    pub fn exec(&self) -> Result<()> {
        let mut command = Command::new("cargo");
        command.arg("build");

        if self.release {
            command.arg("release");
        }

        if let Some((target, linker)) = &self.target {
            command.args(["--target", target]);

            if let Some(linker) = linker {
                command.env(
                    format!(
                        "CARGO_TARGET_{}_LINKER",
                        target.to_uppercase().replace("-", "_")
                    ),
                    linker,
                );
            }
        }

        command.status()?;
        Ok(())
    }
}

pub fn cargo_build() {}
