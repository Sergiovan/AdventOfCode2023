mod problem;
mod day;

use clap::Parser;
use std::io::stdin;
use arboard::Clipboard;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long)]
    day: u64,
    #[arg(long)]
    part: u8
}

fn main() {
    let args = Args::parse();

    let input = stdin().lines().flatten().collect::<Vec<_>>();

    let result = day::run_task(input, args.day, args.part);

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(&result).unwrap();

    println!("Day {}, Part {}: {}", args.day, args.part, result);
}
