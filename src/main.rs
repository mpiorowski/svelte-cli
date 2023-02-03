use anyhow::Result;
use clap::Parser;
use sveltecli::svelte::{set_templates_path, Svelte, create_page};
use sveltecli::{
    opts::Opts,
    setup::{Operation, Setup},
};

fn main() -> Result<()> {
    let opts = Setup::try_from(Opts::parse())?;
    let svelte = Svelte::from_setup(opts)?;
    let pwd = &svelte.setup.pwd;
    let templates_path = &svelte.templates_path;
    println!("{:?}", svelte);

    match svelte.setup.operation {
        Operation::Print(cli) => {
            println!("Print: {:?}", cli);
        }
        Operation::Pages(pages) => {
            std::fs::create_dir_all(pwd)?;
            for page in pages {
                create_page(page, &pwd, &templates_path)?;
            }
        }
        Operation::ConfigTemplatesPath(path) => {
            set_templates_path(&svelte.config_path, &path)?;
        }
    }
    Ok(())
}
