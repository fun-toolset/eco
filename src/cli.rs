use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "TEXT")]
    pub text: Vec<String>,
}
