use crate::opts::Action;
use crate::opts::Opts;
use crate::templates::error_svelte;
use crate::templates::layout_client;
use crate::templates::layout_server;
use crate::templates::layout_svelte;
use crate::templates::page_client;
use crate::templates::page_server;
use crate::templates::page_svelte;
use crate::templates::server;
use ::anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct Setup {
    pub action: Action,
    pub operation: Operation,
    pub pwd: PathBuf,
}

impl TryFrom<Opts> for Setup {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = Operation::try_from(&value.action)?;
        let pwd = PathBuf::try_from(&value.action)?;

        Ok(Setup {
            action: value.action,
            operation,
            pwd,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Pages(Vec<Page>),
    ConfigTemplatesPath(PathBuf),
}

#[derive(Debug)]
pub enum Page {
    E,
    L,
    Lc,
    Ls,
    P,
    Pc,
    Ps,
    S,
}

impl Page {
    pub fn all() -> Vec<&'static str> {
        vec!["e", "l", "lc", "ls", "p", "pc", "ps", "s"]
    }

    pub fn get_page(&self) -> &'static str {
        match self {
            Page::E => "+error.svelte",
            Page::L => "+layout.svelte",
            Page::Lc => "+layout.ts",
            Page::Ls => "+layout.server.ts",
            Page::P => "+page.svelte",
            Page::Pc => "+page.ts",
            Page::Ps => "+page.server.ts",
            Page::S => "+server.ts",
        }
    }

    pub fn get_content(&self) -> &'static str {
        match self {
            Page::E => error_svelte(),
            Page::L => layout_svelte(),
            Page::Lc => layout_client(),
            Page::Ls => layout_server(),
            Page::P => page_svelte(),
            Page::Pc => page_client(),
            Page::Ps => page_server(),
            Page::S => server(),
        }
    }
}

impl FromStr for Page {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "e" => Ok(Page::E),
            "l" => Ok(Page::L),
            "lc" => Ok(Page::Lc),
            "ls" => Ok(Page::Ls),
            "p" => Ok(Page::P),
            "pc" => Ok(Page::Pc),
            "ps" => Ok(Page::Ps),
            "s" => Ok(Page::S),
            _ => Err(anyhow::anyhow!(
                "Invalid value {}. Allowed values are: {:?}",
                s,
                Page::all()
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
                    Page::from_str(arg)?;
                }
                Ok(Operation::Pages(
                    values
                        .args
                        .iter()
                        .map(|v| Page::from_str(v).unwrap())
                        .collect(),
                ))
            }
            Action::Config(values) => {
                if values.key == "templates_path" {
                    return Ok(Operation::ConfigTemplatesPath(PathBuf::from(&values.value)));
                }
                return Err(anyhow::anyhow!("Invalid key"));
            }
        }
    }
}

impl TryFrom<&Action> for PathBuf {
    type Error = anyhow::Error;

    fn try_from(value: &Action) -> Result<Self> {
        match value {
            Action::Add(values) => get_pwd(values.pwd.clone()),
            Action::Config(_) => get_pwd(None),
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
