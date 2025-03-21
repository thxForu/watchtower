# Watchtower

A monitoring tool that uses Telegram's scheduled messages to detect server downtime.

## How it works

The bot continuously reschedules a message to be sent in the future for each monitored user. If the server goes down, it will stop rescheduling, and the message will be sent to the specified Telegram users, effectively alerting them about the server downtime.

This approach provides a reliable "dead man's switch" mechanism:
- While the server is running: The bot continuously reschedule the message
- If server fails: The message gets delivered, indicating system failure
- No false positives: Only triggers when the server actually stops working

## Configuration Before Building

The configuration values are compiled into the binary during build time. You need to set them up before building:

1. Create `config.json` in the project root:
```json
{
    "api_id": your_api_id,
    "api_hash": "your_api_hash",
    "session_file": "watchtower.session",
    "schedule_interval": 15,
    "sleep_interval": 10
}
```

2. Get your API credentials from https://my.telegram.org/apps and update the config

## Building

For Linux (static binary):
```bash
# Install MUSL and build
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```
For Windows:
```bash
# Install MinGW and build
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Running

Simply run the binary for your platform:

Linux
```bash
$ ./watchtower-linux-x64 --users=user1,user2
```

Windows
```bash
watchtower-win-x64.exe --users=user1,user2
```

### Linux. Run in the background.
To run this program in the background in Linux you can use systemd, screen, nohup command & etc.

To run this program in the background using __nohup command &__, follow these steps:

1. Run the binary for your platform for the first time.
   You will need to enter your phone number and the code received in
   Telegram. The program will create the __watchtower.session__ file with
   information about the session. Then stop the program.

Linux:
```bash
$ ./watchtower-linux-x64 --users=user1,user2
Enter the phone number:
+123456789
Enter the code received in Telegram:
12345
Successfully singed in!
Saving session
# stop programm here
```

2. Now you can run this program in the background using __nohup__ bash command.

Linux:
```bash
$ nohup ./watchtower-linux-x64 --users=user1,user2 &
```

This command starts a process that will ignore the HUP signal and will not stop after the user logs off.

## Development

1. Install Rust
2. Clone repository
3. Create and fill `config.json` with your credentials
4. Run `cargo build`

## License

MIT