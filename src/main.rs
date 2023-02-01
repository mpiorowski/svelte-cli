use anyhow::Result;
use clap::Parser;
use svelte_cli::{
    config::{Config, Operation},
    opts::Opts,
};

fn main() -> Result<()> {
    let opts = Config::try_from(Opts::parse())?;
    println!("{:?}", opts);

    match opts.operation {
        Operation::Print(term) => {
            println!("Print: {:?}", term);
        }
        Operation::Pages(name) => {
            println!("Pages: {:?}", name);
            // let full_pwd = opts.pwd;
            // std::fs::create_dir_all(&full_pwd)?;
            // std::fs::write(full_pwd.join("page.server.ts"), page_server())?;
        }
    }
    Ok(())
}
