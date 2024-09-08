use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(long, short)]
    pub webhook_url: String,
    #[arg(long, short)]
    pub message: String,
    #[arg(long, short)]
    pub username: String,
}