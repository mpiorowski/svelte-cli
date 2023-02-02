use std::io::Write;
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
                        let mut file = std::fs::File::create(&full_pwd.join("+error.svelte"))?;
                        file.write_all(error_svelte().as_bytes())?;
                    }
                    AllowedValues::L => {
                        let mut file = std::fs::File::create(&full_pwd.join("+layout.svelte"))?;
                        file.write_all(layout_svelte().as_bytes())?;
                    }
                    AllowedValues::Lc => {
                        let mut file = std::fs::File::create(&full_pwd.join("+layout.ts"))?;
                        file.write_all(layout_client().as_bytes())?;
                    }
                    AllowedValues::Ls => {
                        let mut file = std::fs::File::create(&full_pwd.join("+layout.server.ts"))?;
                        file.write_all(layout_server().as_bytes())?;
                    }
                    AllowedValues::P => {
                        let mut file = std::fs::File::create(&full_pwd.join("+page.svelte"))?;
                        file.write_all(page_svelte().as_bytes())?;
                    }
                    AllowedValues::Pc => {
                        let mut file = std::fs::File::create(&full_pwd.join("+page.ts"))?;
                        file.write_all(page_client().as_bytes())?;
                    }
                    AllowedValues::Ps => {
                        let mut file = std::fs::File::create(&full_pwd.join("+page.server.ts"))?;
                        file.write_all(page_server().as_bytes())?;
                    }
                    AllowedValues::S => {
                        let mut file = std::fs::File::create(&full_pwd.join("server.ts"))?;
                        file.write_all(server().as_bytes())?;
                    }
                }
            }
        }
    }
    Ok(())
}
