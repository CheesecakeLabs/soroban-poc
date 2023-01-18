use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::str;
#[derive(Debug, Serialize, Deserialize)]
struct CarData {
    color: Vec<u8>,
    horse: u8,
    model: Vec<u8>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];

    let data: CarData = serde_json::from_str(query)?;

    println!("Model: {}", str::from_utf8(&data.model).unwrap());
    println!("Color: {}", str::from_utf8(&data.color).unwrap());
    println!("Horse: {}", data.horse);

    Ok(())
}
