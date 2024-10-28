use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn read_dataset(path: &str) -> Vec<HashMap<String, String>> {
    // Get the directory of the current executable
    let script_dir = env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(|p| p.to_path_buf()))
        .expect("Failed to get script directory");

    let dataset_path = script_dir.join(path);

    // Open the CSV file
    let file = File::open(&dataset_path).expect("Failed to open CSV file");

    // Create CSV Reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let headers = rdr.headers().expect("Failed to read CSV headers").clone();

    // Read all records into a vector
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        let mut map = HashMap::new();
        for (key, value) in headers.iter().zip(record.iter()) {
            map.insert(key.to_string(), value.to_string());
        }
        records.push(map);
    }

    records
}

fn statistics(records: &[HashMap<String, String>]) {
    if records.is_empty() {
        println!("No data available for statistics.");
        return;
    }

    // Get all column names
    let columns: Vec<String> = records[0].keys().cloned().collect();

    // Initialize statistics data for each column
    let mut stats: HashMap<String, Vec<f64>> = HashMap::new();

    for column in &columns {
        stats.insert(column.clone(), Vec::new());
    }

    // Collect numeric data
    for record in records {
        for (key, value) in record {
            if let Ok(num) = value.parse::<f64>() {
                stats.get_mut(key).unwrap().push(num);
            }
        }
    }

    // Calculate and print statistics information
    for (column, values) in &stats {
        if values.is_empty() {
            println!("Column '{}' is not numeric, unable to compute statistics.", column);
            continue;
        }

        let count = values.len() as f64;
        let sum: f64 = values.iter().sum();
        let mean = sum / count;
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let variance = values.iter().map(|value| {
            let diff = mean - *value;
            diff * diff
        }).sum::<f64>() / count;
        let std_dev = variance.sqrt();

        println!("Column '{}':", column);
        println!("  Count: {}", count as usize);
        println!("  Mean: {:.4}", mean);
        println!("  Min: {:.4}", min);
        println!("  Max: {:.4}", max);
        println!("  Std Dev: {:.4}", std_dev);
    }
}

fn main() {
    let dataset_path = "../../../ai_job_market_insights.csv";
    let records = read_dataset(dataset_path.as_ref());
    statistics(&records);
}
