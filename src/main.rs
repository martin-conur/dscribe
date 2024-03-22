#![allow(unused)]
use anyhow::{Error as E, Result};
use clap::{Parser, Subcommand, ValueEnum};
use polars::prelude::*;
use polars_core::prelude::*;
use polars_io::prelude::*;

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

    let df: PolarsResult<DataFrame> = {
        CsvReader::from_path(args.path)
            .unwrap()
            .has_header(true)
            .finish()
    };

    match args.operation {
        Operation::Head => {
            println!("{:?}", df.unwrap().head(Some(5)));
        }
        Operation::Mean => {
            let df_mean = df.unwrap().lazy().select([col("*").mean()]).collect();
            println!("{:?}", df_mean)
        }
        Operation::Count => {
            let df_count = df.unwrap().lazy().select([col("*").count()]).collect();
            println!("{:?}", df_count)
        }
        _ => todo!(),
    }
}
