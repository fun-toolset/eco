mod cli;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    // Join the text parts (or strings) with spaces in between
    let output = cli.text.join(" ");
    println!("{}", output);
}
