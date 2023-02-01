use svelte_cli::temp::page_server::page_server;
use anyhow::Result;
use clap::Parser;
use svelte_cli::{
    config::{Svelte, Operation},
    opts::Opts
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
                if page == "l" {
                    std::fs::write(full_pwd.join("layout.svelte"), page_server())?;
                    continue;
                }
                if page == "ls" {
                    std::fs::write(full_pwd.join("layout.server.ts"), page_server())?;
                    continue;
                }
                if page == "p" {
                    std::fs::write(full_pwd.join("page.svelte"), page_server())?;
                    continue;
                }
                if page == "ps" {
                    std::fs::write(full_pwd.join("page.server.ts"), page_server())?;
                    continue;
                }
                if page == "s" {
                    std::fs::write(full_pwd.join("server.ts"), page_server())?;
                    continue;
                }
            }
        }
    }
    Ok(())
}
