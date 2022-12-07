use hex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("{}", hex::encode(query));
}
