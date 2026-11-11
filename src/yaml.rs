use anyhow::Result;
use indexmap::IndexMap;
use serde::Serialize;

pub enum AppExecutionType {
    Local,
    Shell { binary: String },
}

pub fn app_yaml(
    app_execution_type: AppExecutionType,
    env_vars: IndexMap<&str, String>,
) -> Result<String> {
    let config = AppConfig {
        command: match app_execution_type {
            AppExecutionType::Local => &["cargo", "run"],
            AppExecutionType::Shell { binary } => &[
                "sh",
                "-c",
                &format!("chmod +x ./{binary} && exec ./{binary}"),
            ],
        },
        env: env_vars
            .iter()
            .map(|(name, value)| EnvVar { name, value })
            .collect(),
    };

    Ok(serde_yaml::to_string(&config)?)
}

#[derive(Serialize)]
struct AppConfig<'a> {
    command: &'a [&'a str],
    env: Vec<EnvVar<'a>>,
}

#[derive(Serialize)]
struct EnvVar<'a> {
    name: &'a str,
    value: &'a str,
}
