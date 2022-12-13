use std::env;
use std::fmt::Write;
use stellar_strkey::*;

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn test_strkey_decode(key: &String) {
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
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    test_strkey_decode(&query);
}
