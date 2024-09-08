use clap::Parser;
use crate::options::Arguments;

mod options;

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let webhook_url = args.webhook_url;
    let message = args.message;


    if let Err(e) = discord_webhook_lib::send_discord(
        webhook_url.as_str(),
        message.as_str(),
        "Botty Guard"
    ).await {
        println!("{}", e);
    }
}
