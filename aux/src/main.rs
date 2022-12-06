use std::fmt::Write;
use std::io::{self};

use stellar_strkey::*;

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn test_strkey_decode(key: String) {
    let str_key = Strkey::from_string(&key).unwrap();
    match str_key {
        Strkey::PublicKeyEd25519(value) => match value {
            stellar_strkey::StrkeyPublicKeyEd25519(v) => println!("{:?}", encode_hex(&v)),
            _ => println!("error"),
        },
        _ => println!("error"),
    }
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    test_strkey_decode(input.trim().to_string());
}
