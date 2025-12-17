use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    #[serde(rename = "sticker_id")]
    pub sticker_id: Option<String>,
    #[serde(rename = "emoji_character")]
    pub emoji_character: Option<String>,
    pub file: File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollStatus {
    pub state: Option<String>,
    #[serde(rename = "selection_index")]
    pub selection_index: Option<i32>,
    #[serde(rename = "percent_vote_options")]
    pub percent_vote_options: Vec<i32>,
    #[serde(rename = "total_vote")]
    pub total_vote: Option<i32>,
    #[serde(rename = "show_total_votes")]
    pub show_total_votes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub question: Option<String>,
    pub options: Vec<String>,
    #[serde(rename = "poll_status")]
    pub poll_status: PollStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveLocation {
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    #[serde(rename = "live_period")]
    pub live_period: Option<i32>,
    #[serde(rename = "current_location")]
    pub current_location: Location,
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "last_update_time")]
    pub last_update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMessage {
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardedFrom {
    #[serde(rename = "type_from")]
    pub type_from: Option<String>,
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    #[serde(rename = "from_chat_id")]
    pub from_chat_id: Option<String>,
    #[serde(rename = "from_sender_id")]
    pub from_sender_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuxData {
    #[serde(rename = "start_id")]
    pub start_id: Option<String>,
    #[serde(rename = "button_id")]
    pub button_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTextbox {
    #[serde(rename = "type_line")]
    pub type_line: Option<String>,
    #[serde(rename = "type_keypad")]
    pub type_keypad: Option<String>,
    #[serde(rename = "place_holder")]
    pub place_holder: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonNumberPicker {
    #[serde(rename = "min_value")]
    pub min_value: Option<String>,
    #[serde(rename = "max_value")]
    pub max_value: Option<String>,
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStringPicker {
    pub items: Vec<String>,
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonCalendar {
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "min_year")]
    pub min_year: Option<String>,
    #[serde(rename = "max_year")]
    pub max_year: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonLocation {
    #[serde(rename = "default_pointer_location")]
    pub default_pointer_location: Location,
    #[serde(rename = "default_map_location")]
    pub default_map_location: Location,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "location_image_url")]
    pub location_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonSelectionItem {
    pub text: Option<String>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonSelection {
    #[serde(rename = "selection_id")]
    pub selection_id: Option<String>,
    #[serde(rename = "search_type")]
    pub search_type: Option<String>,
    #[serde(rename = "get_type")]
    pub get_type: Option<String>,
    pub items: Vec<ButtonSelectionItem>,
    #[serde(rename = "is_multi_selection")]
    pub is_multi_selection: Option<bool>,
    #[serde(rename = "columns_count")]
    pub columns_count: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "button_text")]
    pub button_text: Option<String>,
    #[serde(rename = "button_selection")]
    pub button_selection: Option<ButtonSelection>,
    #[serde(rename = "button_calendar")]
    pub button_calendar: Option<ButtonCalendar>,
    #[serde(rename = "button_number_picker")]
    pub button_number_picker: Option<ButtonNumberPicker>,
    #[serde(rename = "button_string_picker")]
    pub button_string_picker: Option<ButtonStringPicker>,
    #[serde(rename = "button_location")]
    pub button_location: Option<ButtonLocation>,
    #[serde(rename = "button_textbox")]
    pub button_textbox: Option<ButtonTextbox>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeypadRow {
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keypad {
    pub rows: Vec<KeypadRow>,
    #[serde(rename = "resize_keyboard")]
    pub resize_keyboard: bool,
    #[serde(rename = "on_time_keyboard")]
    pub on_time_keyboard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename = "chat_id")]
    pub chat_id: Option<String>,
    #[serde(rename = "chat_type")]
    pub chat_type: Option<String>,
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    #[serde(rename = "bot_id")]
    pub bot_id: Option<String>,
    #[serde(rename = "bot_title")]
    pub bot_title: Option<String>,
    pub avatar: File,
    pub description: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "start_message")]
    pub start_message: Option<String>,
    #[serde(rename = "share_url")]
    pub share_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub bot: Arc<crate::api::Robot>,
    pub chat_id: String,
    pub message_id: String,
    pub sender_id: String,
    pub text: Option<String>,
    pub raw_data: serde_json::Value,
    pub time: Option<String>,
    pub is_edited: bool,
    pub sender_type: Option<String>,
    pub args: Vec<String>,
    pub reply_to_message_id: Option<String>,
    pub forwarded_from: Option<ForwardedFrom>,
    pub file: Option<File>,
    pub sticker: Option<Sticker>,
    pub contact_message: Option<ContactMessage>,
    pub poll: Option<Poll>,
    pub location: Option<Location>,
    pub live_location: Option<LiveLocation>,
    pub aux_data: Option<AuxData>,
}

impl Message {
    pub fn new(
        bot: Arc<crate::api::Robot>,
        chat_id: String,
        message_id: String,
        sender_id: String,
        text: Option<String>,
        raw_data: Option<serde_json::Value>,
    ) -> Self {
        let raw = raw_data.unwrap_or_default();
        Message {
            bot,
            chat_id,
            message_id: raw.get("message_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or(message_id),
            sender_id: raw.get("sender_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or(sender_id),
            text: raw.get("text")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or(text),
            raw_data: raw.clone(),
            time: raw.get("time").and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_edited: raw.get("is_edited").and_then(|v| v.as_bool()).unwrap_or(false),
            sender_type: raw.get("sender_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
            args: vec![],
            reply_to_message_id: raw.get("reply_to_message_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            forwarded_from: raw.get("forwarded_from")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            file: raw.get("file")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            sticker: raw.get("sticker")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            contact_message: raw.get("contact_message")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            poll: raw.get("poll")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            location: raw.get("location")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            live_location: raw.get("live_location")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            aux_data: raw.get("aux_data")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
        }
    }

    pub fn session(&self) -> parking_lot::RwLock<HashMap<String, serde_json::Value>> {
        parking_lot::RwLock::new(HashMap::new())
    }

    pub async fn reply(&self, text: &str) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_message(
            &self.chat_id,
            text,
            None,
            None,
            false,
            Some(&self.message_id),
            None,
        ).await
    }

    pub async fn reply_poll(
        &self,
        question: &str,
        options: &[String],
    ) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_poll(&self.chat_id, question, options).await
    }

    pub async fn reply_location(
        &self,
        latitude: &str,
        longitude: &str,
    ) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_location(
            &self.chat_id,
            latitude,
            longitude,
            false,
            None,
            Some(&self.message_id),
            None,
        ).await
    }

    pub async fn reply_contact(
        &self,
        first_name: &str,
        last_name: &str,
        phone_number: &str,
    ) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_contact(
            &self.chat_id,
            first_name,
            last_name,
            phone_number,
        ).await
    }

    pub async fn reply_keypad(
        &self,
        text: &str,
        keypad: &serde_json::Value,
    ) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_message(
            &self.chat_id,
            text,
            Some(keypad),
            None,
            false,
            Some(&self.message_id),
            Some("New"),
        ).await
    }

    pub async fn reply_inline(
        &self,
        text: &str,
        inline_keypad: &serde_json::Value,
    ) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.send_message(
            &self.chat_id,
            text,
            None,
            Some(inline_keypad),
            false,
            Some(&self.message_id),
            None,
        ).await
    }

    pub async fn edit(&self, new_text: &str) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.edit_message_text(&self.chat_id, &self.message_id, new_text).await
    }

    pub async fn delete(&self) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.delete_message(&self.chat_id, &self.message_id).await
    }
}

#[derive(Debug, Clone)]
pub struct InlineMessage {
    pub bot: Arc<crate::api::Robot>,
    pub raw_data: serde_json::Value,
    pub chat_id: String,
    pub message_id: String,
    pub sender_id: String,
    pub text: Option<String>,
    pub aux_data: Option<AuxData>,
}

impl InlineMessage {
    pub fn new(bot: Arc<crate::api::Robot>, raw_data: serde_json::Value) -> Self {
        InlineMessage {
            bot,
            chat_id: raw_data.get("chat_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            message_id: raw_data.get("message_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            sender_id: raw_data.get("sender_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            text: raw_data.get("text")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            aux_data: raw_data.get("aux_data")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            raw_data,
        }
    }

    pub async fn delete(&self) -> Result<serde_json::Value, crate::exceptions::APIRequestError> {
        self.bot.delete_message(&self.chat_id, &self.message_id).await
    }
}

