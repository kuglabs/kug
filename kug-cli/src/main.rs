use clap::Parser;
use kug_core::core_functionality;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello from {}!", &args.name);
    core_functionality();
}
