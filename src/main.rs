mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
}
