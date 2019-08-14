//! Sensor information
use glob::glob;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Sensor {
    pub name: String,
    pub temperatures: Vec<Temperature>,
}

// TODO temperature
#[derive(Debug)]
pub struct Temperature {
    pub label: String,
    pub current: f32,
    pub high: f32,
    pub critical: Option<f32>,
}

pub fn sensors_temperature() -> std::io::Result<Vec<Sensor>> {
    let s = read_hwmon();
    println!("{:#?}", s);
    s
}

/// Reads all entries found in /sys/class/hwmon and parses the sensors and the temperatures
fn read_hwmon() -> io::Result<Vec<Sensor>> {
    fs::read_dir("/sys/class/hwmon/").and_then(|dir| {
        dir.map(|entry| entry.and_then(|e| parse_sensor(e)))
            .collect()
    })
}

/// Given a sensor, parses the sensor information and the temperature
fn parse_sensor(entry: fs::DirEntry) -> io::Result<Sensor> {
    let sensor_name = fs::read_to_string(file_in(&entry, "name"))?;
    let temps = read_temps(&entry)?;
    Ok(Sensor {
        name: sensor_name.trim().to_string(),
        temperatures: temps,
    })
}

fn read_temps(entry: &fs::DirEntry) -> io::Result<Vec<Temperature>> {
    let temperatures = format!("{}/temp*_input", entry.path().display());
    glob(&temperatures).map(|paths: glob::Paths| {
        paths
            .map(|glob_result: glob::GlobResult| glob_result.map(|glob| read_temp(&glob)))
            .collect::<Vec<_>>()
    });
    Ok(vec![])
}

fn read_temp(input_temp: &PathBuf) -> Temperature {
    dbg!(input_temp);
    Temperature {
        label: "".to_string(),
        current: 0.0,
        high: 0.0,
        critical: Some(0.0),
    }
}

fn file_in<'a>(entry: &'a fs::DirEntry, file: &'a str) -> PathBuf {
    let mut base = entry.path();
    base.push(file);
    base
}

// TODO fans

// batteries are already taken care of by https://github.com/svartalf/rust-battery
