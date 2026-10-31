use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("CHRtool...");
    println!("Building CHR from: {} {}", &args[0], &args[1]);
}
