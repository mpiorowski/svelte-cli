use crate::setup::Page;
use crate::setup::Setup;
use anyhow::Context;
use anyhow::Result;
use serde_json::from_str;
use serde_json::to_string;
use serde_json::to_string_pretty;
use serde_json::Value;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Svelte {
    pub setup: Setup,
    pub config_path: PathBuf,
    pub templates_path: Option<PathBuf>,
    pub language: String,
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
                language: "ts".to_string(),
            });
        }

        let config = std::fs::read_to_string(&config_path).context("Config not found")?;
        let config: serde_json::Value =
            serde_json::from_str(&config).context("Config not valid")?;
        let templates_path = config
            .get("temp")
            .and_then(|v| v.as_str())
            .map(|v| PathBuf::from(v));
        let language = config
            .get("lang")
            .and_then(|v| v.as_str())
            .unwrap_or("ts")
            .to_owned();
        return Ok(Svelte {
            setup,
            config_path,
            templates_path,
            language,
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
    let mut config_json: Value = from_str(&config_string).context("Config not valid")?;
    let templates_str = &path.to_str().context("Path not valid")?;
    let templates_json = to_string(&templates_str).context("Json not valid")?;
    config_json["temp"] = from_str(&templates_json).context("Json not valid")?;

    config_string = to_string_pretty(&config_json).context("Json not valid")?;
    let mut file = std::fs::File::create(config_path).context("File not found")?;
    file.write_all(config_string.as_bytes())?;

    return Ok(());
}

pub fn set_language(config_path: &PathBuf, lang: &str) -> Result<()> {
    let mut config_string = std::fs::read_to_string(config_path).context("Config not found")?;
    let mut config_json: Value = from_str(&config_string).context("Config not valid")?;
    config_json["lang"] = lang.into();

    config_string = to_string_pretty(&config_json).context("Json not valid")?;
    let mut file = std::fs::File::create(config_path).context("File not found")?;
    file.write_all(config_string.as_bytes())?;

    return Ok(());
}

pub fn create_page(
    page: Page,
    pwd: &PathBuf,
    templates_path: &Option<PathBuf>,
    language: &String,
) -> Result<()> {
    let page_str = Page::get_page(&page, language);
    let page_content: String;

    if let Some(val) = templates_path {
        let file_path = val.join(page_str);
        if std::fs::metadata(&file_path).is_err() {
            page_content = String::from(Page::get_content(&page, language));
        } else {
            let mut file = std::fs::File::open(&file_path).context("File not found")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            page_content = contents.into();
        }
    } else {
        page_content = Page::get_content(&page, language).into();
    }

    let mut file =
        std::fs::File::create(&pwd.join(page_str.to_owned())).context("File not found")?;
    file.write_all(page_content.as_bytes())?;

    return Ok(());
}
