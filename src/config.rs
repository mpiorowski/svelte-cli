use crate::opts::Action;
use crate::opts::Opts;
use ::anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;
use std::str::FromStr;

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
    Pages(Vec<AllowedValues>),
}

#[derive(Debug)]
pub enum AllowedValues {
    L,
    Ls,
    P,
    Ps,
    S,
}

impl AllowedValues {
    pub fn all() -> Vec<&'static str> {
        vec!["l", "ls", "p", "ps", "s"]
    }
}

impl FromStr for AllowedValues {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "l" => Ok(AllowedValues::L),
            "ls" => Ok(AllowedValues::Ls),
            "p" => Ok(AllowedValues::P),
            "ps" => Ok(AllowedValues::Ps),
            "s" => Ok(AllowedValues::S),
            _ => Err(anyhow::anyhow!("Invalid value {}. Allowed values are: {:?}", s, AllowedValues::all())),
        }
    }
}

impl TryFrom<&Action> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: &Action) -> Result<Self> {
        match value {
            Action::Add(values) => {
                if values.args.is_empty() {
                    return Err(anyhow::anyhow!("no args"));
                }
                for arg in &values.args {
                    AllowedValues::from_str(arg)?;
                }
                Ok(Operation::Pages(values.args.iter().map(|v| AllowedValues::from_str(v).unwrap()).collect()))
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
