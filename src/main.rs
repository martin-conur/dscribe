#![allow(unused)]
#[macro_use]
extern crate prettytable;

use std::path::PathBuf;

use anyhow::{Error as E, Result};
use clap::{Parser, Subcommand, ValueEnum};
use csv::{Reader, StringRecord};

use prettytable::{Cell, Row, Table};

impl Args {
    fn columns(&self) -> Result<StringRecord, csv::Error> {
        let mut rdr = Reader::from_path(&self.path)?;
        let r = rdr.headers()?.clone();
        Ok(r)
    }
}

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Format {
    Csv,
    // TO DO: IMPLEMENT
    // Txt,
    // Paquet,
    // Excel,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Operation {
    /// show top 5 rows
    Head,
    /// show the summary of every column
    Summary,
    ///
    Nan,
    /// counts the files not nans values
    NotNan,
    /// get columns names
    Columns,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// File path to read
    #[arg(required = true)]
    path: std::path::PathBuf,

    /// File's format
    #[arg(long, short, value_enum, default_value_t=Format::Csv, global=true)]
    format: Format,

    /// File delimiter, valid only for csv and txt formats
    #[arg(long, short, global=true, default_value_t=String::from(","))]
    delimiter: String,

    /// Type of operation to perform on the file's columns
    #[arg(value_enum, default_value_t=Operation::Head)]
    operation: Operation,
}

fn main() {
    let args = Args::parse();
}
