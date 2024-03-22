#![allow(unused)]
use anyhow::{Error as E, Result};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Format {
    Csv,
    Txt,
    Paquet,
    Excel,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// file path to read
    #[arg(required = true)]
    path: std::path::PathBuf,

    /// file format
    #[arg(long, short, value_enum, default_value_t=Format::Csv, global=true)]
    format: Format,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);
}
