use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Action {
    Add(Add),
}

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(
    about = "Svelte CLI",
    long_about = "Svelte CLI is a command line interface for Svelte prototyping and development."
)]
pub struct Opts {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Args, Debug)]
pub struct Add {
    pub args: Vec<String>,

    #[arg(short = 'p', long = "pwd")]
    pub pwd: Option<PathBuf>,
}
