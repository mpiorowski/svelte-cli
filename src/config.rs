use crate::opts::Opts;
use ::anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,
    pub pwd: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = Operation::try_from(value.args)?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Config {
            operation,
            config,
            pwd,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Page(String, String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self> {
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).unwrap().to_string();
        if term == "add" {
            if value.len() < 3 {
                return Err(anyhow::anyhow!("Page requires a name and a path"));
            }
            let mut drain = value.drain(1..=2);

            Ok(Operation::Page(
                drain.next().expect("page name"),
                drain.next().expect("page path"),
            ))
        } else {
            Ok(Operation::Print(Some(term)))
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
