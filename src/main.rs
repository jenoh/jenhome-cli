use clap::Parser;

#[derive(Parser)]
struct Cli {
    light: String,
    status: String,
}

fn main() {
    let args = Cli::parse();
    if args.status == String::from("on") || args.status == String::from("off") {
        println!("ok")
    } else {
        println!("Only on and off are avaiblable")
    }
}
