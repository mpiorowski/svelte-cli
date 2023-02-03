use crate::setup::Page;
use crate::setup::Setup;
use anyhow::Context;
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Svelte {
    pub setup: Setup,
    pub config_path: PathBuf,
    pub templates_path: Option<PathBuf>,
}

impl Svelte {
    pub fn from_setup(setup: Setup) -> Result<Svelte> {
        let config_path = get_config_path().context("Config not found")?;

        if std::fs::metadata(&config_path).is_err() {
            let mut file = std::fs::File::create(&config_path).context("File not found")?;
            file.write_all(b"{\"lang\": \"ts\"}")?;
            return Ok(Svelte {
                setup,
                config_path,
                templates_path: None,
            });
        }

        let config = std::fs::read_to_string(&config_path).context("Config not found")?;
        let config: serde_json::Value =
            serde_json::from_str(&config).context("Config not valid")?;
        let templates_path = config
            .get("templates_path")
            .and_then(|v| v.as_str())
            .map(|v| PathBuf::from(v));
        return Ok(Svelte {
            setup,
            config_path,
            templates_path,
        });
    }
}

fn get_config_path() -> Result<PathBuf> {
    let loc = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|v| v + "/.config"))
        .context("Config not set")?;
    let mut loc = PathBuf::from(loc);
    loc.push("svelte-cli.json");
    return Ok(loc);
}

pub fn set_templates_path(config_path: &PathBuf, path: &PathBuf) -> Result<()> {
    let mut config_string = std::fs::read_to_string(config_path).context("Config not found")?;
    let mut config_json: serde_json::Value =
        serde_json::from_str(&config_string).context("Config not valid")?;
    let templates_str = &path.to_str().context("Path not valid")?;
    let templates_json = serde_json::to_string(&templates_str).context("Json not valid")?;
    config_json["templates_path"] =
        serde_json::from_str(&templates_json).context("Json not valid")?;

    config_string = serde_json::to_string_pretty(&config_json).context("Json not valid")?;
    let mut file = std::fs::File::create(config_path).context("File not found")?;
    file.write_all(config_string.as_bytes())?;

    return Ok(());
}

pub fn create_page(page: Page, pwd: &PathBuf, templates_path: &Option<PathBuf>) -> Result<()> {
    let page_str = Page::get_page(&page);
    let page_content: String;

    if let Some(val) = templates_path {
        let file_path = val.join(page_str);
        let file_content = std::fs::read_to_string(file_path).context("File not found")?;
        page_content = file_content;
    } else {
        page_content = String::from(Page::get_content(&page));
    }

    let mut file = std::fs::File::create(&pwd.join(page_str)).context("File not found")?;
    file.write_all(page_content.as_bytes())?;

    return Ok(());
}
