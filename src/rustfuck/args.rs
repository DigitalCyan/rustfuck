use clap::Parser;

#[derive(Parser)]
pub struct RFArgs {
    /// File to interpret
    #[arg(short, long)]
    pub path: String,
}