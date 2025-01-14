use crate::auth::Auth;
use crate::constants::CONSTANTS;
use grammers_client::{Client, Config, InitParams, InputMessage};
use grammers_session::{PackedChat, Session};
use rustyline::DefaultEditor;
use std::error::Error;
use std::time::SystemTime;
use tokio::time::sleep;

pub struct TelegramBot {
    auth: Auth,
}

impl TelegramBot {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config {
            api_id: CONSTANTS.api_id,
            api_hash: CONSTANTS.api_hash.to_string(),
            session: Session::load_file_or_create(CONSTANTS.session_file)?,
            params: InitParams {
                catch_up: true,
                ..InitParams::default()
            },
        };

        let client = Client::connect(config).await?;
        let auth = Auth::new(client);

        Ok(Self { auth })
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.auth.ensure_authorized().await?;

        let usernames = self.get_usernames()?;
        if usernames.is_empty() {
            println!("No usernames provided. Exiting...");
            return Ok(());
        }

        println!("Starting message scheduling for users: {:?}", usernames);
        self.schedule_messages_for_users(&usernames).await?;
        Ok(())
    }

    fn get_usernames(&self) -> Result<Vec<String>, Box<dyn Error>> {
        println!("Enter usernames (comma-separated):");
        let mut editor = DefaultEditor::new()?;
        let input = editor.readline("> ")?;

        let usernames: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(usernames)
    }

    async fn schedule_messages_for_users(
        &self,
        usernames: &[String],
    ) -> Result<(), Box<dyn Error>> {
        let mut tasks = Vec::new();
        let client = self.auth.get_client();

        for username in usernames {
            let client_clone = client.clone();
            let username_clone = username.clone();
            let schedule_interval = CONSTANTS.schedule_interval;
            let sleep_interval = CONSTANTS.sleep_interval;

            let handle = tokio::spawn(async move {
                if let Err(e) = Self::handle_user_messages(
                    client_clone,
                    &username_clone,
                    schedule_interval,
                    sleep_interval,
                )
                .await
                {
                    eprintln!("Error handling messages for {}: {:?}", username_clone, e);
                }
            });
            tasks.push(handle);
        }

        for task in tasks {
            if let Err(e) = task.await {
                eprintln!("Task error: {:?}", e);
            }
        }
        Ok(())
    }

    async fn handle_user_messages(
        client: Client,
        username: &str,
        schedule_interval: u64,
        sleep_interval: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let chat = match client.resolve_username(username).await {
            Ok(Some(chat)) => chat.pack(),
            Ok(None) => return Err(format!("User {} not found", username).into()),
            Err(e) => return Err(format!("Error resolving username {}: {:?}", username, e).into()),
        };

        let message = Self::create_scheduled_message(schedule_interval);
        let sent_message = client.send_message(chat, message).await?;

        Self::run_reschedule_loop(
            client,
            chat,
            sent_message.id(),
            username,
            schedule_interval,
            sleep_interval,
        )
        .await
    }

    fn create_scheduled_message(delay_seconds: u64) -> InputMessage {
        let schedule_time = SystemTime::now() + std::time::Duration::from_secs(delay_seconds);

        let current_time = chrono::Local::now();
        let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S %Z").to_string();

        let message = format!(
            "🚨 SERVER DOWN ALERT!\n\n\
            ⚠️ The server monitoring system has stopped responding!\n\
            📅 Last alive: {}\n\n
            This message indicates that the server has stopped responding \
            and requires immediate attention.",
            formatted_time
        );

        InputMessage::text(message).schedule_date(Some(schedule_time))
    }

    async fn run_reschedule_loop(
        client: Client,
        chat: PackedChat,
        message_id: i32,
        username: &str,
        schedule_interval: u64,
        sleep_interval: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            let message = Self::create_scheduled_message(schedule_interval);
            client.edit_message(chat, message_id, message).await?;

            let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
            println!(
                "[{}] Heartbeat sent for {} (next check in {} seconds)",
                current_time, username, schedule_interval
            );

            sleep(tokio::time::Duration::from_secs(sleep_interval)).await;
        }
    }
}
