mod auth;
mod bot;
mod constants;

use clap::Parser;
use bot::TelegramBot;

#[derive(Parser)]
#[command(arg_required_else_help = true, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    users: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let bot = TelegramBot::new().await?;
    bot.run(&args.users).await?;
    Ok(())
}
