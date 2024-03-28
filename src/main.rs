#![allow(unused)]
use std::path::PathBuf;

use anyhow::{Error as E, Result};
use clap::{Parser, Subcommand, ValueEnum};
use csv::{Reader, StringRecord};
use polars::prelude::*;
use polars_core::prelude::*;
use polars_io::{predicates::ColumnStats, prelude::*};

fn read_csv_columns(path: &PathBuf) -> Result<StringRecord, csv::Error> {
    let mut rdr = Reader::from_path(path)?;
    let r = rdr.headers()?;
    Ok(r.clone())
}

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Format {
    Csv,
    Txt,
    Paquet,
    Excel,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Operation {
    /// show all the rows
    ShowAll,
    /// show top 5 rows
    Head,
    /// show the mean of each numeric column
    Mean,
    /// show the median of each numeric column
    Median,
    /// show the most repeated value of each column
    Mode,
    /// sum all numeric values across column axis
    Sum,
    /// counts the file's rows
    Count,
    /// counts the file's nan values
    Nan,
    /// counts the files not nans values
    NotNan,
    /// perform all above basic statistics
    BasicStatistics,
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

    let df = LazyCsvReader::new(&args.path)
        .has_header(true)
        .finish()
        .unwrap();

    match args.operation {
        Operation::Head => {
            println!("{:?}", df.collect().unwrap().head(Some(5)));
        }
        Operation::Mean => {
            let df_mean = df.select([col("*").mean()]).collect().unwrap();
            println!("{:?}", df_mean)
        }
        Operation::Count => {
            let df_count = df.select([col("*").count()]).collect().unwrap();
            println!("{:?}", df_count)
        }
        Operation::Columns => {
            println!(
                "{:?}",
                read_csv_columns(&args.path).expect("error reading file")
            );
        }
        Operation::Nan => {
            let nans = df.select([col("*").is_null().sum()]).collect().unwrap();

            println!("NaNs in file:");
            println!("{:?}", nans)
        }
        Operation::NotNan => {
            let not_nans = df.select([col("*").is_not_null().sum()]).collect().unwrap();

            println!("Not NaNs in file:");
            println!("{:?}", not_nans);
        }
        _ => todo!(),
    }
}
