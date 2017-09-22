extern crate base64;

use std::u8;
use self::base64::encode;

#[allow(dead_code)]
pub fn hex_to_64(hex: String) -> String {
    
    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(t) => bytes.push(t),
            Err(e) => println!("Error: {}",e),
        };
    }

    encode(&bytes)
}