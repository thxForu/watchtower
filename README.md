# Watchtower

A monitoring tool that uses Telegram's scheduled messages to detect server downtime.

## How it works

The bot continuously reschedules a message to be sent in the future for each monitored user. If the server goes down, it will stop rescheduling, and the message will be sent to the specified Telegram users, effectively alerting them about the server downtime.

This approach provides a reliable "dead man's switch" mechanism:
- While the server is running: The bot continuously postpones the message
- If server fails: The message gets delivered, indicating system failure
- No false positives: Only triggers when the server actually stops working

## Setup

1. Copy configuration file:
```bash
cp config.example.json config.json
```

2. Edit `config.json` and fill in your Telegram API credentials:
- Get your API credentials from https://my.telegram.org/apps
- Update api_id and api_hash in config.json

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/watchtower
```

## Configuration

The bot can be configured using either:
- `config.json` file in the working directory
- Custom config path via environment variable: `CONFIG_PATH=/path/to/config.json ./watchtower`

## Development

1. Install Rust
2. Clone repository
3. Run `cargo build`

## License

MIT