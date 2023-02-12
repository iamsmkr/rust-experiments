use chrono::{Datelike, NaiveDate};
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "data/data-usage.txt"]
        .iter()
        .collect();

    fn parse_record(rec: &StringRecord) -> Option<(NaiveDate, f64, String)> {
        let date = rec.get(0).and_then(|s| s.parse::<String>().ok())?;
        let date = NaiveDate::parse_from_str(&date, "%d/%m/%Y").unwrap();
        let quantity = rec.get(2).and_then(|s| s.parse::<f64>().ok())?;
        let unit = rec.get(3).and_then(|s| s.parse::<String>().ok())?;
        Some((date, quantity, unit))
    }

    let mut usage: HashMap<(i32, u32), f64> = HashMap::new();

    if let Ok(mut reader) = ReaderBuilder::new().delimiter(b' ').from_path(&file_path) {
        for rec_res in reader.records() {
            if let Ok(rec) = rec_res {
                if let Some((date, mut quantity, unit)) = parse_record(&rec) {
                    let year = date.year();
                    let month = date.month();

                    match unit.as_str() {
                        "B" => quantity = quantity * 0.001 * 0.001 * 0.001,
                        "KB" => quantity = quantity * 0.001 * 0.001,
                        "MB" => quantity = quantity * 0.001,
                        _ => panic!("Unit not recognised"),
                    }

                    let total_usage = usage.entry((year, month)).or_insert(0f64);
                    *total_usage += quantity;
                }
            }
        }
    }

    for ((y, m), u) in usage {
        println!("{} {}: {:.2} GB", y, m, u);
    }
}
