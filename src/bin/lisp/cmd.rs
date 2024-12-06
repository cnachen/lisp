use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Parse input only
    #[arg(short, long)]
    pub parse: bool,
}
