mod set1;

fn main() {
    let start : String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("hex:\n {}\n", start );
    let b64 = set1::chal1::hex_to_64(start);
    println!("b46: \n {} \n", b64);
}