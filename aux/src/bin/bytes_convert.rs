use json::{self, JsonValue};
use std::str;

fn main() {
    // some bytes, in a vector
    // let mut intVar: u8 = 71;
    // let mut charVar: char;

    // charVar = intVar as char;
    // println!("Character is : {}", charVar);

    // let var = vec![71, 111, 108];
    // let args: Vec<String> = env::args().collect();
    // let query = &args[1];

    // We know these bytes are valid, so just use unwrap().
    // let var = str::from_utf8(&var).unwrap();

    let parsed = json::parse(
        r#"

    {"color":[114,101,100],"horse":32,"model":[71,111,108]}

    "#,
    )
    .unwrap();

    let vec: Vec<u32> = parsed["model"]
        .into_iter()
        .map(|value| json::from_value(value).unwrap())
        .collect();

    match var {
        JsonValue::Array(value) => println!("{:?}", str::from_utf8(&value.to_).unwrap()),
        _ => println!("error"),
    }

    // let var = Vec::from(parsed["model"].clone());
    // println!("{}", str::from_utf8(&var).unwrap());
}
