mod lib;
use std::env;

use lib::getflag;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = String::from(&args[1].to_uppercase());

    let result = getflag(&word);
    match result {
        Some(flord) => {
            println!("Congratulations! It's a flord:\n{flord}");
        }
        None => println!("not a flord :("),
    }
}
