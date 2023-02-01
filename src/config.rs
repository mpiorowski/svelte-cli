use crate::opts::Action;
use crate::opts::Opts;
use ::anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub action: Action,
    pub operation: Operation,
    pub config: PathBuf,
    pub pwd: PathBuf,
}

struct OperationBuilder<'a> {
    action: &'a Action,
    value: Vec<String>,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = Operation::try_from(OperationBuilder {
            action: &value.action,
            value: value.action
        })?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Config {
            action: value.action,
            operation,
            config,
            pwd,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Pages(Vec<String>),
}

const ALLOWED_VALUES: [&str; 3] = ["value1", "value2", "value3"];

impl TryFrom<OperationBuilder<'_>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: OperationBuilder) -> Result<Self> {
        if value.value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let action = value.action;
        if let Action::Add = action {
            return Ok(Operation::Pages(value.value));
        } else {
            Ok(Operation::Print(None))
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
