use clap::Parser;
use svelte_cli::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
