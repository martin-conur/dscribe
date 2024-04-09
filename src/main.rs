#![allow(unused)]
use std::fmt::Debug;
use std::sync::Arc;

#[macro_use]
use std::path::PathBuf;
use arrow::datatypes::{DataType, Field, Schema};
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
    /// show N top rows
    Show { n: usize },
    /// sample N rows
    Sample,
    /// show the summary of every column
    Summary,
    /// Use SQL to query the file, the table name is 'input' by default
    Sql { query: String },
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

    let schema = arrow_csv::reader::infer_schema_from_files(
        &[args.path.to_str().unwrap().to_string()],
        args.delimiter.as_bytes()[0],
        Some(1000),
        args.header,
    )?;

    let mut fields = schema
        .fields()
        .iter()
        .map(|field| {
            let field_name = field.name().to_ascii_lowercase().replace(' ', "_");
            let datatype = field.data_type();
            let nullable = field.is_nullable();
            Field::new(field_name, datatype.clone(), nullable)
        })
        .collect::<Vec<Field>>();

    let schema = Arc::new(Schema::new(fields));

    let ctx = SessionContext::new();
    let options = CsvReadOptions::new()
        .delimiter(args.delimiter.as_bytes()[0])
        .has_header(args.header)
        .schema(&schema);

    // register csv file with the execution context
    ctx.register_csv("input", args.path.to_str().unwrap(), options)
        .await?;

    let df = ctx.table("input").await?;

    let df = match args.operation {
        Operation::Head => df.limit(0, Some(5))?,
        Operation::Show { n: val } => df.limit(0, Some(val))?,
        Operation::Summary => df.describe().await.unwrap(),
        Operation::Sql { query } => ctx.sql(&query).await?,
        Operation::Columns => {
            println!("{}", df.schema());
            todo!()
        }
        _ => todo!(),
    };

    let result = df.collect().await?;

    let pretty_results = arrow::util::pretty::pretty_format_batches(&result)?;

    println!("{}", pretty_results);
    Ok(())
}
