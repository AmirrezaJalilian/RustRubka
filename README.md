# 📚 rust_rubka Bot Rust Library

A Rust library for interacting with the [Rubika Bot API](https://rubika.ir/). This is a Rust port of the Python `rubka` library.

## ⚙️ Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_rubka = { version = "0.1.0", path = "." }
```

Or from crates.io (when published):

```toml
[dependencies]
rust_rubka = "0.1.0"
```

## 🚀 Getting Started

```rust
use rust_rubka::{Robot, Message};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Arc::new(Robot::new(
        "YOUR_TOKEN_HERE".to_string(),
        None,
        None,
        None,
        None,
        None,
    ));

    let bot_clone = bot.clone();
    bot.on_message(None, Some(vec!["start".to_string()]), move |bot, msg| {
        let bot = bot.clone();
        let msg = msg.clone();
        tokio::spawn(async move {
            let _ = msg.reply("سلام! خوش آمدید!").await;
        });
    });

    bot.run().await?;
    Ok(())
}
```

## 📬 Handling Messages

You can handle incoming text messages using `on_message()`:

```rust
use rust_rubka::{Robot, Message};
use std::sync::Arc;

let bot = Arc::new(Robot::new("TOKEN".to_string(), None, None, None, None, None));

bot.on_message(None, Some(vec!["hello".to_string()]), move |bot, msg| {
    let bot = bot.clone();
    let msg = msg.clone();
    tokio::spawn(async move {
        let _ = msg.reply("سلام کاربر عزیز 👋").await;
    });
});
```

## 🎮 Handling Callback Buttons

```rust
use rust_rubka::{Robot, Message, ChatKeypadBuilder};
use std::sync::Arc;

let bot = Arc::new(Robot::new("TOKEN".to_string(), None, None, None, None, None));

let bot_clone = bot.clone();
bot.on_message(None, Some(vec!["gender".to_string()]), move |bot, msg| {
    let bot = bot.clone();
    let msg = msg.clone();
    tokio::spawn(async move {
        let builder = ChatKeypadBuilder::new();
        let btn1 = builder.button("male", "👨 مرد", None);
        let btn2 = builder.button("female", "👩 زن", None);
        let keypad = builder.row(&[btn1, btn2]).build(None, None);
        
        let _ = msg.reply_keypad("جنسیت خود را انتخاب کنید:", &keypad).await;
    });
});

let bot_clone2 = bot.clone();
bot.on_callback(Some("male".to_string()), move |bot, msg| {
    let msg = msg.clone();
    tokio::spawn(async move {
        let _ = msg.reply("شما مرد هستید").await;
    });
});

let bot_clone3 = bot.clone();
bot.on_callback(Some("female".to_string()), move |bot, msg| {
    let msg = msg.clone();
    tokio::spawn(async move {
        let _ = msg.reply("شما زن هستید").await;
    });
});
```

## 🔘 Inline Button Builder

```rust
use rust_rubka::InlineBuilder;

let builder = InlineBuilder::new();
let btn = builder.button_simple("info", "اطلاعات");
let inline_keypad = builder.row(&[btn]).build();
```

## 💬 Utility Methods

| Method | Description |
|--------|-------------|
| `get_chat(chat_id)` | Get chat information |
| `get_name(chat_id)` | Get user name |
| `get_username(chat_id)` | Get username |
| `send_message(...)` | Send text message |
| `edit_message_text(...)` | Edit message |
| `delete_message(...)` | Delete message |
| `send_location(...)` | Send location |
| `send_poll(...)` | Send poll |
| `send_contact(...)` | Send contact |
| `forward_message(...)` | Forward message |

## 🧱 Button Types

Supported inline button types include:

- `Simple`
- `Payment`
- `Calendar`
- `Location`
- `CameraImage`, `CameraVideo`
- `GalleryImage`, `GalleryVideo`
- `File`, `Audio`, `RecordAudio`
- `MyPhoneNumber`, `MyLocation`
- `Textbox`, `Barcode`, `Link`

## 🧩 Dynamic Chat Keypad

```rust
use rust_rubka::ChatKeypadBuilder;

let builder = ChatKeypadBuilder::new();
let btn1 = builder.button("play", "🎮 بازی کن", None);
let btn2 = builder.button("exit", "❌ خروج", None);
let keypad = builder.row(&[btn1, btn2]).build(None, None);
```

## 🧪 Set Commands

```rust
use serde_json::json;

let commands = vec![
    json!({"command": "start", "description": "شروع"}),
    json!({"command": "help", "description": "راهنما"}),
];
let _ = bot.set_commands(&commands).await;
```

## 🔄 Update Offset Automatically

Bot updates are handled using `get_updates()` and `offset_id` is managed internally in the `run()` method.

## 🛠 Advanced Features

- `update_bot_endpoint()` – Set webhook or polling
- `remove_keypad()` – Remove chat keypad
- `edit_chat_keypad()` – Edit or add chat keypad

## Differences from Python Version

1. **Async/Await**: All API methods are async in Rust
2. **Handlers**: Message handlers run in spawned tasks
3. **Types**: Strong typing with Rust's type system
4. **Error Handling**: Uses `Result<T, E>` instead of exceptions
5. **Ownership**: Uses `Arc` for shared ownership of the Robot instance

## License

This project is licensed under the MIT License.
