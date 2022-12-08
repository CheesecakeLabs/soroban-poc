use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    color: Vec<u8>,
}

fn from_bytes_to_str(bytes: Vec<u8>) {
    println!("{}", str::from_utf8(&bytes).unwrap())
}

fn main() -> Result<()> {
    let json = r#"{
        "color":[114,101,100],"horse":32,"model":[71,111,108]
    }"#;
    let data: Data = serde_json::from_str(json)?;
    println!("{:?}", data.color);

    from_bytes_to_str(data.color);
    Ok(())
}
