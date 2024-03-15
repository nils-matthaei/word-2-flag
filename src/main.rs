use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
}
