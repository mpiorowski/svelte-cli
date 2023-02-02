use anyhow::Result;
use clap::Parser;
use sveltecli::templates::{
    error_svelte, layout_client, layout_server, layout_svelte, page_client, page_server,
    page_svelte, server,
};
use sveltecli::{
    config::{Operation, Svelte},
    opts::Opts,
};
use sveltecli::config::AllowedValues;

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
                    AllowedValues::E => {
                        std::fs::write(&full_pwd.join("+error.svelte"), error_svelte())?;
                    }
                    AllowedValues::L => {
                        std::fs::write(&full_pwd.join("+layout.svelte"), layout_svelte())?;
                    }
                    AllowedValues::Lc => {
                        std::fs::write(&full_pwd.join("+layout.ts"), layout_client())?;
                    }
                    AllowedValues::Ls => {
                        std::fs::write(&full_pwd.join("+layout.server.ts"), layout_server())?;
                    }
                    AllowedValues::P => {
                        std::fs::write(&full_pwd.join("+page.svelte"), page_svelte())?;
                    }
                    AllowedValues::Pc => {
                        std::fs::write(&full_pwd.join("+page.ts"), page_client())?;
                    }
                    AllowedValues::Ps => {
                        std::fs::write(&full_pwd.join("+page.server.ts"), page_server())?;
                    }
                    AllowedValues::S => {
                        std::fs::write(&full_pwd.join("+server.ts"), server())?;
                    }
                }
            }
        }
    }
    Ok(())
}
