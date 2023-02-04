use anyhow::Result;
use clap::Parser;
use sveltecli::svelte::{set_templates_path, Svelte, create_page, set_language};
use sveltecli::{
    opts::Opts,
    setup::{Operation, Setup},
};

fn main() -> Result<()> {
    let opts = Setup::try_from(Opts::parse())?;
    let svelte = Svelte::from_setup(opts)?;
    let pwd = &svelte.setup.pwd;
    let templates_path = &svelte.templates_path;
    let language = &svelte.language;

    match svelte.setup.operation {
        Operation::Print => {
            println!("Print: {:?}", svelte);
        }
        Operation::Pages(pages) => {
            std::fs::create_dir_all(pwd)?;
            for page in pages {
                create_page(page, &pwd, &templates_path, language)?;
            }
        }
        Operation::ConfigTemplatesPath(path) => {
            set_templates_path(&svelte.config_path, &path)?;
        }
        Operation::ConfigLanguage(lang) => {
            set_language(&svelte.config_path, &lang)?;
        }
    }
    Ok(())
}
