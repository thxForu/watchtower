mod auth;
mod bot;
mod constants;

use bot::TelegramBot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = TelegramBot::new().await?;
    bot.run().await?;
    Ok(())
}
