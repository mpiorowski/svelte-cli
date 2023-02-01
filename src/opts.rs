use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Action {
    Add(Add),
    Remove,
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

    #[arg(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    #[arg(short = 'p', long = "pwd")]
    pub pwd: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct Add {
    pub args: Vec<String>,
}
