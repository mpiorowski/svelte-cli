use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use crate::setup::Setup;

pub struct Config {
    pub setup: Setup,
    pub data: serde_json::Value,
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
