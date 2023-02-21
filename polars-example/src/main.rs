use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;

fn main() -> Result<()> {
    // sample_csv()

    read_lotr()
}

fn read_lotr() -> Result<()> {
    let df = CsvReader::from_path("data/lotr.csv")?
        .has_header(false)
        .finish()?;

    assert!(!df.is_empty());

    println!("{:?}", df);

    Ok(())
}

fn sample_csv() -> Result<()> {
    let data: Vec<u8> = Client::new()
        .get("https://j.mp/iriscsv")
        .send()?
        .text()?
        .bytes()
        .collect();

    let df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        .filter(col("sepal_length").gt(5))
        .groupby([col("species")])
        .agg([col("*").sum()])
        .collect()?;

    println!("{:?}", df);

    Ok(())
}
