use clap::Parser;
use samuel::morse::translate;

pub mod cli;

fn main() {
    let args = cli::Args::parse();
    let translated_string = translate(&args.translate);
    println!("{}", translated_string);
}
