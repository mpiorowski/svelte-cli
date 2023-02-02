use anyhow::Result;
use clap::Parser;
use svelte_cli::config::AllowedValues;
use svelte_cli::templates::{page_server, layout_server, page_svelte, layout_svelte, server};
use svelte_cli::{
    config::{Operation, Svelte},
    opts::Opts,
};

fn main() -> Result<()> {
    let opts = Svelte::try_from(Opts::parse())?;
    println!("{:?}", opts);

    match opts.operation {
        Operation::Print(cli) => {
            println!("Print: {:?}", cli);
        }
        Operation::Pages(pages) => {
            let full_pwd = opts.pwd;
            std::fs::create_dir_all(&full_pwd)?;
            for page in pages {
                match page {
                    AllowedValues::L => {
                        std::fs::write(&full_pwd.join("layout.svelte"), layout_svelte())?;
                    }
                    AllowedValues::Ls => {
                        std::fs::write(&full_pwd.join("layout.server.ts"), layout_server())?;
                    }
                    AllowedValues::P => {
                        std::fs::write(&full_pwd.join("page.svelte"), page_svelte())?;
                    }
                    AllowedValues::Ps => {
                        std::fs::write(&full_pwd.join("page.server.ts"), page_server())?;
                    }
                    AllowedValues::S => {
                        std::fs::write(&full_pwd.join("server.ts"), server())?;
                    }
                }
            }
        }
    }
    Ok(())
}
