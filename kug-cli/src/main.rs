use clap::Parser;
use kug_utils::greet;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    greet(&args.name);
}
