use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    timestamp: String,
    led_num: u32,
    driver_number: u32,
}

#[derive(Debug)]
struct DriverData {
    driver_number: u32,
    led_num: u32,
}

#[derive(Debug)]
struct UpdateFrame {
    drivers: Vec<DriverData>,
}

#[derive(Debug)]
struct VisualizationData {
    update_rate_ms: u32,
    frames: Vec<UpdateFrame>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let file = File::open("output_track_data_short_sample_consolidated_timestamps.csv")?;
    let reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);

    // HashMap to store driver data for each timestamp
    let mut timestamp_map: HashMap<String, HashMap<u32, u32>> = HashMap::new();

    // Process the CSV records
    for result in rdr.deserialize() {
        let record: Record = result?;
        let entry = timestamp_map
            .entry(record.timestamp)
            .or_insert_with(HashMap::new);
        entry.insert(record.led_num, record.driver_number);
    }

    // Create frames from the HashMap and sort by timestamp
    let mut frames = Vec::new();
    let mut sorted_timestamps: Vec<_> = timestamp_map.keys().collect();
    sorted_timestamps.sort();

    for timestamp in sorted_timestamps {
        if let Some(drivers_map) = timestamp_map.get(timestamp) {
            let mut drivers = Vec::new();
            for (led_num, driver_number) in drivers_map {
                drivers.push(DriverData {
                    driver_number: *driver_number,
                    led_num: *led_num,
                });
            }
            frames.push(UpdateFrame { drivers });
        }
    }

    // Create the VisualizationData structure
    let visualization_data = VisualizationData {
        update_rate_ms: 100,
        frames,
    };

    // Write the output to a file
    let mut output_file = File::create("output.txt")?;
    writeln!(output_file, "{:#?}", visualization_data)?;

    Ok(())
}
