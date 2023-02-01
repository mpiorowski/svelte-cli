use crate::opts::Action;
use crate::opts::Opts;
use ::anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Svelte {
    pub action: Action,
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Svelte {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = Operation::try_from(&value.action)?;
        let pwd = get_pwd(value.pwd)?;
        let config = get_config(value.config)?;

        Ok(Svelte {
            action: value.action,
            operation,
            pwd,
            config,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Pages(Vec<String>),
}

const ALLOWED_VALUES: [&str; 5] = ["l", "ls", "p", "ps", "s"];

impl TryFrom<&Action> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: &Action) -> Result<Self> {
        match value {
            Action::Add(values) => {
                for v in &values.args {
                    if !ALLOWED_VALUES.contains(&v.as_str()) {
                        return Err(anyhow::anyhow!(
                            "{} is not a valid value. Allowed values are: {:?}",
                            v,
                            ALLOWED_VALUES
                        ));
                    }
                }
                Ok(Operation::Pages(values.args.clone()))
            }
        }
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|v| v + "/.config"))
        .context("config not set")?;

    let mut loc = PathBuf::from(loc);

    loc.push("svelte-cli.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }
    let loc = std::env::current_dir().context("pwd not set")?;
    return Ok(loc);
}
