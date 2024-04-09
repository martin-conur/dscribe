#![allow(unused)]
#[macro_use]
use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use datafusion::datasource::file_format::file_compression_type::FileCompressionType;
use datafusion::error::Result;
use datafusion::prelude::*;

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Format {
    Csv,
    // TO DO: IMPLEMENT
    // Txt,
    // Paquet,
    // Excel,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// show top 5 rows
    Head,
    /// show the summary of every column
    Summary,
    /// Use SQL to query the file
    Query { query: String },
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
    #[arg(long, short, value_enum, global=true, default_value_t=Format::Csv)]
    format: Format,

    /// File delimiter, valid only for csv and txt formats
    #[arg(long, short, global=true, default_value_t=String::from(","))]
    delimiter: String,

    /// Whether the file has header or not.
    #[arg(long, default_value_t = true)]
    header: bool,

    /// Type of operation to perform on the file's columns
    #[command(subcommand)]
    operation: Operation,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let ctx = SessionContext::new();
    let options = CsvReadOptions::new()
        .delimiter(args.delimiter.as_bytes()[0])
        .has_header(args.header);

    let df = ctx.read_csv(args.path.to_str().unwrap(), options).await?;

    let result = df.collect().await?;

    let pretty_results = arrow::util::pretty::pretty_format_batches(&result)?;

    println!("{}", pretty_results);
    Ok(())
}
