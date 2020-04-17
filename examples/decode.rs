use tilejson::decode;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide file with json as parameter.");
        return;
    }
    let json = fs::read_to_string(&args[1]).unwrap();

    let tile = decode(&json);
    println!("{:?}", tile);
}