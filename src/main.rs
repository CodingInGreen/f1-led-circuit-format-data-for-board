use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
pub struct DriverData {
    pub driver_number: u32,
    pub led_num: u32,
}

#[derive(Debug)]
pub struct UpdateFrame {
    pub drivers: [Option<DriverData>; 20],
}

#[derive(Debug)]
pub struct VisualizationData {
    pub update_rate_ms: u32,
    pub frames: [UpdateFrame; 1548],
}

#[derive(Debug)]
pub struct DriverDataWithTimestamp {
    pub timestamp: String,
    pub driver_data: DriverData,
}

fn main() {
    let file = File::open("output_track_data_short_sample_consolidated_timestamps.csv").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut driver_data_vec: Vec<DriverDataWithTimestamp> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue; // skip header
        }
        let line = line.expect("Cannot read line");
        let fields: Vec<&str> = line.split(',').collect();
        let timestamp: String = fields[0].trim().to_string();
        let driver_number: u32 = fields[2].trim().parse().expect("Cannot parse driver_number");
        let led_num: u32 = fields[1].trim().parse().expect("Cannot parse led_num");
        driver_data_vec.push(DriverDataWithTimestamp {
            timestamp,
            driver_data: DriverData {
                driver_number,
                led_num,
            },
        });
    }

    // Sort the data by timestamp
    driver_data_vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Group data by timestamp
    let mut timestamp_map: HashMap<String, Vec<DriverData>> = HashMap::new();
    for entry in driver_data_vec {
        let entry_list = timestamp_map.entry(entry.timestamp).or_insert(Vec::new());
        if !entry_list.iter().any(|d| d.driver_number == entry.driver_data.driver_number) {
            entry_list.push(entry.driver_data);
        }
    }

    let mut frames: Vec<UpdateFrame> = Vec::new();
    let total_frames = 1548;

    for (_, driver_data_list) in timestamp_map.into_iter() {
        let mut drivers: [Option<DriverData>; 20] = Default::default();
        for (i, driver_data) in driver_data_list.iter().take(20).enumerate() {
            drivers[i] = Some(*driver_data);
        }
        frames.push(UpdateFrame { drivers });

        // If we reach the total number of frames needed, stop processing
        if frames.len() == total_frames {
            break;
        }
    }

    // Fill remaining frames with empty UpdateFrames if necessary
    while frames.len() < total_frames {
        frames.push(UpdateFrame { drivers: [None; 20] });
    }

    let visualization_data = VisualizationData {
        update_rate_ms: 1000,
        frames: frames.try_into().expect("Wrong number of frames"),
    };

    let mut output_file = File::create("output.txt").expect("Cannot create output file");
    write!(
        output_file,
        "pub const VISUALIZATION_DATA: VisualizationData = {:?};",
        visualization_data
    )
    .expect("Cannot write to output file");
}
