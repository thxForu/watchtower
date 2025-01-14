use crate::constants::CONSTANTS;
use grammers_client::{Client, SignInError};
use rpassword::read_password;
use rustyline::DefaultEditor;
use std::error::Error;

#[derive(Debug)]
pub struct Auth {
    client: Client,
}

#[derive(Debug)]
pub struct AuthError(String);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authentication error: {}", self.0)
    }
}

impl Error for AuthError {}

impl Auth {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn ensure_authorized(&self) -> Result<(), Box<dyn Error>> {
        if self.client.is_authorized().await? {
            return Ok(());
        }

        self.authenticate().await
    }

    async fn authenticate(&self) -> Result<(), Box<dyn Error>> {
        let phone = Self::prompt_input("Enter the phone number:")?;
        let token = self.client.request_login_code(&phone).await?;

        let code = Self::prompt_input("Enter the code received in Telegram:")?;

        match self.client.sign_in(&token, &code).await {
            Ok(_) => {
                println!("Successfully signed in!");
                self.save_session()?;
                Ok(())
            }
            Err(SignInError::PasswordRequired(password_token)) => {
                self.handle_2fa(password_token).await
            }
            Err(e) => Err(Box::new(AuthError(e.to_string()))),
        }
    }

    async fn handle_2fa(
        &self,
        password_token: grammers_client::types::PasswordToken,
    ) -> Result<(), Box<dyn Error>> {
        let hint = password_token.hint().unwrap_or("None");
        print!("Enter the password (hint {}): ", hint);
        std::io::Write::flush(&mut std::io::stdout())?;

        let password = read_password()?;

        self.client
            .check_password(password_token, &password)
            .await?;
        self.save_session()?;
        Ok(())
    }

    fn save_session(&self) -> Result<(), Box<dyn Error>> {
        println!("Saving session");
        self.client.session().save_to_file(CONSTANTS.session_file)?;
        Ok(())
    }

    fn prompt_input(prompt: &str) -> Result<String, Box<dyn Error>> {
        println!("{}", prompt);
        let mut editor = DefaultEditor::new()?;
        let line = editor.readline("")?;
        Ok(line.trim().to_string())
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }
}
