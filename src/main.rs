use anyhow::Result;
use clap::Parser;
use svelte::{
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
        Operation::Page(name, path) => {
            let full_pwd = opts.pwd.join(&path);
            std::fs::create_dir_all(&full_pwd)?;
            std::fs::copy("temp/page.server.ts", full_pwd.join("page.server.ts"))?;
            println!("Page: {} {}", name, path);
        }
    }
    Ok(())
}
