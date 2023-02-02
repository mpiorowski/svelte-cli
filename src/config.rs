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
}

impl TryFrom<Opts> for Svelte {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = Operation::try_from(&value.action)?;
        let pwd = PathBuf::try_from(&value.action)?;

        Ok(Svelte {
            action: value.action,
            operation,
            pwd,
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
    E,
    L,
    Lc,
    Ls,
    P,
    Pc,
    Ps,
    S,
}

impl AllowedValues {
    pub fn all() -> Vec<&'static str> {
        vec!["e", "l", "lc", "ls", "p", "pc", "ps", "s"]
    }
}

impl FromStr for AllowedValues {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "e" => Ok(AllowedValues::E),
            "l" => Ok(AllowedValues::L),
            "lc" => Ok(AllowedValues::Lc),
            "ls" => Ok(AllowedValues::Ls),
            "p" => Ok(AllowedValues::P),
            "pc" => Ok(AllowedValues::Pc),
            "ps" => Ok(AllowedValues::Ps),
            "s" => Ok(AllowedValues::S),
            _ => Err(anyhow::anyhow!(
                "Invalid value {}. Allowed values are: {:?}",
                s,
                AllowedValues::all()
            )),
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
                Ok(Operation::Pages(
                    values
                        .args
                        .iter()
                        .map(|v| AllowedValues::from_str(v).unwrap())
                        .collect(),
                ))
            }
        }
    }
}

impl TryFrom<&Action> for PathBuf {
    type Error = anyhow::Error;

    fn try_from(value: &Action) -> Result<Self> {
        match value {
            Action::Add(values) => get_pwd(values.pwd.clone()),
        }
    }
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }
    let loc = std::env::current_dir().context("pwd not set")?;
    return Ok(loc);
}
