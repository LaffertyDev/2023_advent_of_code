mod problems;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: usize,

    input: std::path::PathBuf
}

fn main() {
    let cli = Args::parse();

    let days = problems::get_days();

    if !cli.input.exists() {
        println!("Input file does not exist. Did you remember to create it? Proceeding.")
    }

    if let Some(day) = days.get(cli.day) {
        day.part1(&cli.input);
        day.part2(&cli.input);
    } else {
        println!("!!! Unknown Day !!! Did you remember to update the problems/mod.rs file?")
    }
}
