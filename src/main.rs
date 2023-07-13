use clap::Parser;

#[derive(Parser)]
struct Cli {
    light: String,
    status: String
}

fn main() {
    let args = Cli::parse();
    // println!(args);
    println!("Hello, world!");
}
