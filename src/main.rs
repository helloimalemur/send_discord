use clap::Parser;
use crate::options::Arguments;

mod options;

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let webhook_url = args.webhook_url;
    let message = args.message;
    let username = args.username;

    if let Err(e) = discord_webhook_lib::send_discord(
        webhook_url.as_str(),
        message.as_str(),
        username.as_str(),
    ).await {
        println!("{}", e);
    }
}
