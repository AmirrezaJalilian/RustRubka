use crate::context::{Message, InlineMessage};
use crate::exceptions::APIRequestError;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use std::path::Path;
use tokio::time::{sleep, Duration};

const API_URL: &str = "https://botapi.rubika.ir/v3";

pub type MessageHandler = Box<dyn Fn(Arc<Robot>, Message) + Send + Sync>;
pub type CallbackHandler = Box<dyn Fn(Arc<Robot>, Message) + Send + Sync>;
pub type InlineQueryHandler = Box<dyn Fn(Arc<Robot>, InlineMessage) + Send + Sync>;

pub struct Robot {
    pub token: String,
    pub timeout: u64,
    pub auth: Option<String>,
    pub session_name: Option<String>,
    pub key: Option<String>,
    pub platform: String,
    pub offset_id: Arc<RwLock<Option<String>>>,
    pub client: reqwest::Client,
    pub sessions: Arc<RwLock<HashMap<String, HashMap<String, Value>>>>,
    pub message_handlers: Arc<RwLock<Vec<MessageHandler>>>,
    pub callback_handlers: Arc<RwLock<Vec<CallbackHandler>>>,
    pub inline_query_handler: Arc<RwLock<Option<InlineQueryHandler>>>,
}

impl Robot {
    pub fn new(
        token: String,
        session_name: Option<String>,
        auth: Option<String>,
        key: Option<String>,
        platform: Option<String>,
        timeout: Option<u64>,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout.unwrap_or(10)))
            .build()
            .expect("Failed to create HTTP client");

        let robot = Robot {
            token: token.clone(),
            timeout: timeout.unwrap_or(10),
            auth,
            session_name,
            key,
            platform: platform.unwrap_or_else(|| "web".to_string()),
            offset_id: Arc::new(RwLock::new(None)),
            client,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: Arc::new(RwLock::new(Vec::new())),
            callback_handlers: Arc::new(RwLock::new(Vec::new())),
            inline_query_handler: Arc::new(RwLock::new(None)),
        };

        crate::logger::log_info(&format!("Initialized RubikaBot with token: {}***", &token[..8.min(token.len())]));
        robot
    }

    pub fn get_session(&self, chat_id: &str) -> Arc<RwLock<HashMap<String, Value>>> {
        let mut sessions = self.sessions.write();
        if !sessions.contains_key(chat_id) {
            sessions.insert(chat_id.to_string(), HashMap::new());
        }
        Arc::new(RwLock::new(sessions.get(chat_id).unwrap().clone()))
    }

    async fn post(&self, method: &str, data: &Value) -> Result<Value, APIRequestError> {
        let url = format!("{}/{}/{}", API_URL, self.token, method);
        let response = self.client
            .post(&url)
            .json(data)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(APIRequestError::RequestFailed(
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())
            ));
        }

        let json_resp: Value = response.json().await?;
        
        if method != "getUpdates" {
            crate::logger::log_debug(&format!("API Response from {}: {}", method, json_resp));
        }
        
        Ok(json_resp)
    }

    pub fn on_message<F>(&self, filters: Option<Box<dyn Fn(&Message) -> bool + Send + Sync>>, commands: Option<Vec<String>>, handler: F)
    where
        F: Fn(Arc<Robot>, Message) + Send + Sync + 'static,
    {
        let handler: MessageHandler = Box::new(move |bot, mut msg| {
            if let Some(ref cmds) = commands {
                if let Some(ref text) = msg.text {
                    if !text.starts_with('/') {
                        return;
                    }
                    let parts: Vec<&str> = text.split_whitespace().collect();
                    if parts.is_empty() {
                        return;
                    }
                    let cmd = &parts[0][1..];
                    if !cmds.contains(&cmd.to_string()) {
                        return;
                    }
                    msg.args = parts[1..].iter().map(|s| s.to_string()).collect();
                }
            }

            if let Some(ref filter) = filters {
                if !filter(&msg) {
                    return;
                }
            }

            handler(bot.clone(), msg);
        });

        self.message_handlers.write().push(handler);
    }

    pub fn on_callback<F>(&self, button_id: Option<String>, handler: F)
    where
        F: Fn(Arc<Robot>, Message) + Send + Sync + 'static,
    {
        let handler: CallbackHandler = Box::new(move |bot, msg| {
            if let Some(ref bid) = button_id {
                if let Some(ref aux) = msg.aux_data {
                    if let Some(ref msg_bid) = aux.button_id {
                        if msg_bid != bid {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            handler(bot.clone(), msg);
        });

        self.callback_handlers.write().push(handler);
    }

    pub fn on_inline_query<F>(&self, handler: F)
    where
        F: Fn(Arc<Robot>, InlineMessage) + Send + Sync + 'static,
    {
        let handler: InlineQueryHandler = Box::new(move |bot, msg| {
            handler(bot.clone(), msg);
        });

        *self.inline_query_handler.write() = Some(handler);
    }

    async fn process_update(&self, update: &Value, bot: Arc<Robot>) {
        if let Some(update_type) = update.get("type").and_then(|v| v.as_str()) {
            if update_type == "ReceiveQuery" {
                if let Some(inline_msg) = update.get("inline_message") {
                    if let Some(handler) = self.inline_query_handler.read().as_ref() {
                        let context = InlineMessage::new(bot.clone(), inline_msg.clone());
                        handler(bot.clone(), context);
                    }
                }
                return;
            }

            if update_type == "NewMessage" {
                if let Some(new_msg) = update.get("new_message") {
                    let chat_id = update.get("chat_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let message_id = new_msg.get("message_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let sender_id = new_msg.get("sender_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let text = new_msg.get("text")
                        .and_then(|v| v.as_str());

                    if let Some(time_str) = new_msg.get("time").and_then(|v| v.as_str()) {
                        if let Ok(msg_time) = time_str.parse::<f64>() {
                            let current_time = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs_f64();
                            if current_time - msg_time > 20.0 {
                                return;
                            }
                        }
                    }

                    let context = Message::new(
                        bot.clone(),
                        chat_id.to_string(),
                        message_id.to_string(),
                        sender_id.to_string(),
                        text.map(|s| s.to_string()),
                        Some(new_msg.clone()),
                    );

                    if context.aux_data.is_some() {
                        for handler in self.callback_handlers.read().iter() {
                            handler(bot.clone(), context.clone());
                            return;
                        }
                    }

                    for handler in self.message_handlers.read().iter() {
                        handler(bot.clone(), context.clone());
                    }
                }
            }
        }
    }

    pub async fn get_me(&self) -> Result<Value, APIRequestError> {
        self.post("getMe", &json!({})).await
    }

    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
        chat_keypad: Option<&Value>,
        inline_keypad: Option<&Value>,
        disable_notification: bool,
        reply_to_message_id: Option<&str>,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut payload = json!({
            "chat_id": chat_id,
            "text": text,
            "disable_notification": disable_notification
        });

        if let Some(ck) = chat_keypad {
            payload["chat_keypad"] = ck.clone();
        }
        if let Some(ik) = inline_keypad {
            payload["inline_keypad"] = ik.clone();
        }
        if let Some(rtmi) = reply_to_message_id {
            payload["reply_to_message_id"] = json!(rtmi);
        }
        if let Some(cktype) = chat_keypad_type {
            payload["chat_keypad_type"] = json!(cktype);
        }

        self.post("sendMessage", &payload).await
    }

    pub async fn send_poll(
        &self,
        chat_id: &str,
        question: &str,
        options: &[String],
    ) -> Result<Value, APIRequestError> {
        self.post("sendPoll", &json!({
            "chat_id": chat_id,
            "question": question,
            "options": options
        })).await
    }

    pub async fn send_location(
        &self,
        chat_id: &str,
        latitude: &str,
        longitude: &str,
        disable_notification: bool,
        inline_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut payload = json!({
            "chat_id": chat_id,
            "latitude": latitude,
            "longitude": longitude,
            "disable_notification": disable_notification
        });

        if let Some(ik) = inline_keypad {
            payload["inline_keypad"] = ik.clone();
        }
        if let Some(rtmi) = reply_to_message_id {
            payload["reply_to_message_id"] = json!(rtmi);
        }
        if let Some(cktype) = chat_keypad_type {
            payload["chat_keypad_type"] = json!(cktype);
        }

        self.post("sendLocation", &payload).await
    }

    pub async fn send_contact(
        &self,
        chat_id: &str,
        first_name: &str,
        last_name: &str,
        phone_number: &str,
    ) -> Result<Value, APIRequestError> {
        self.post("sendContact", &json!({
            "chat_id": chat_id,
            "first_name": first_name,
            "last_name": last_name,
            "phone_number": phone_number
        })).await
    }

    pub async fn get_chat(&self, chat_id: &str) -> Result<Value, APIRequestError> {
        self.post("getChat", &json!({"chat_id": chat_id})).await
    }

    pub async fn get_upload_url(&self, media_type: &str) -> Result<String, APIRequestError> {
        let allowed = vec!["File", "Image", "Voice", "Music", "Gif"];
        if !allowed.contains(&media_type) {
            return Err(APIRequestError::RequestFailed(
                format!("Invalid media type. Must be one of {:?}", allowed)
            ));
        }
        let result = self.post("requestSendFile", &json!({"type": media_type})).await?;
        Ok(result.get("data")
            .and_then(|d| d.get("upload_url"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string())
    }

    pub async fn upload_media_file(
        &self,
        upload_url: &str,
        name: &str,
        path: &str,
    ) -> Result<String, APIRequestError> {
        let file_bytes = if path.starts_with("http") {
            let response = self.client.get(path).send().await?;
            response.bytes().await?.to_vec()
        } else {
            tokio::fs::read(path).await
                .map_err(|e| APIRequestError::RequestFailed(format!("Failed to read file: {}", e)))?
        };

        let mut form = reqwest::multipart::Form::new();
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(name.to_string())
            .mime_str("application/octet-stream")?;
        form = form.part("file", part);

        let response = self.client
            .post(upload_url)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(APIRequestError::RequestFailed(
                format!("Upload failed: {}", response.status())
            ));
        }

        let data: Value = response.json().await?;
        Ok(data.get("data")
            .and_then(|d| d.get("file_id"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string())
    }

    async fn send_uploaded_file(
        &self,
        chat_id: &str,
        file_id: &str,
        text: Option<&str>,
        chat_keypad: Option<&Value>,
        inline_keypad: Option<&Value>,
        disable_notification: bool,
        reply_to_message_id: Option<&str>,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut payload = json!({
            "chat_id": chat_id,
            "file_id": file_id,
            "disable_notification": disable_notification,
            "chat_keypad_type": chat_keypad_type.unwrap_or("None")
        });

        if let Some(t) = text {
            payload["text"] = json!(t);
        }
        if let Some(ck) = chat_keypad {
            payload["chat_keypad"] = ck.clone();
        }
        if let Some(ik) = inline_keypad {
            payload["inline_keypad"] = ik.clone();
        }
        if let Some(rtmi) = reply_to_message_id {
            payload["reply_to_message_id"] = json!(rtmi);
        }

        self.post("sendFile", &payload).await
    }

    pub async fn send_document(
        &self,
        chat_id: &str,
        path: Option<&str>,
        file_id: Option<&str>,
        text: Option<&str>,
        file_name: Option<&str>,
        inline_keypad: Option<&Value>,
        chat_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        disable_notification: bool,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut final_file_id = file_id.map(|s| s.to_string());

        if let Some(p) = path {
            let name = file_name.unwrap_or_else(|| {
                Path::new(p).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
            });
            let upload_url = self.get_upload_url("File").await?;
            final_file_id = Some(self.upload_media_file(&upload_url, name, p).await?);
        }

        if final_file_id.is_none() {
            return Err(APIRequestError::RequestFailed(
                "Either path or file_id must be provided".to_string()
            ));
        }

        self.send_uploaded_file(
            chat_id,
            &final_file_id.unwrap(),
            text,
            chat_keypad,
            inline_keypad,
            disable_notification,
            reply_to_message_id,
            chat_keypad_type,
        ).await
    }

    pub async fn send_image(
        &self,
        chat_id: &str,
        path: Option<&str>,
        file_id: Option<&str>,
        text: Option<&str>,
        file_name: Option<&str>,
        inline_keypad: Option<&Value>,
        chat_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        disable_notification: bool,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut final_file_id = file_id.map(|s| s.to_string());

        if let Some(p) = path {
            let name = file_name.unwrap_or_else(|| {
                Path::new(p).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("image.jpg")
            });
            let upload_url = self.get_upload_url("Image").await?;
            final_file_id = Some(self.upload_media_file(&upload_url, name, p).await?);
        }

        if final_file_id.is_none() {
            return Err(APIRequestError::RequestFailed(
                "Either path or file_id must be provided".to_string()
            ));
        }

        self.send_uploaded_file(
            chat_id,
            &final_file_id.unwrap(),
            text,
            chat_keypad,
            inline_keypad,
            disable_notification,
            reply_to_message_id,
            chat_keypad_type,
        ).await
    }

    pub async fn send_music(
        &self,
        chat_id: &str,
        path: Option<&str>,
        file_id: Option<&str>,
        text: Option<&str>,
        file_name: Option<&str>,
        inline_keypad: Option<&Value>,
        chat_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        disable_notification: bool,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut final_file_id = file_id.map(|s| s.to_string());

        if let Some(p) = path {
            let name = file_name.unwrap_or_else(|| {
                Path::new(p).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("music.mp3")
            });
            let upload_url = self.get_upload_url("Music").await?;
            final_file_id = Some(self.upload_media_file(&upload_url, name, p).await?);
        }

        if final_file_id.is_none() {
            return Err(APIRequestError::RequestFailed(
                "Either path or file_id must be provided".to_string()
            ));
        }

        self.send_uploaded_file(
            chat_id,
            &final_file_id.unwrap(),
            text,
            chat_keypad,
            inline_keypad,
            disable_notification,
            reply_to_message_id,
            chat_keypad_type,
        ).await
    }

    pub async fn send_voice(
        &self,
        chat_id: &str,
        path: Option<&str>,
        file_id: Option<&str>,
        text: Option<&str>,
        file_name: Option<&str>,
        inline_keypad: Option<&Value>,
        chat_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        disable_notification: bool,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut final_file_id = file_id.map(|s| s.to_string());

        if let Some(p) = path {
            let name = file_name.unwrap_or_else(|| {
                Path::new(p).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("voice.ogg")
            });
            let upload_url = self.get_upload_url("Voice").await?;
            final_file_id = Some(self.upload_media_file(&upload_url, name, p).await?);
        }

        if final_file_id.is_none() {
            return Err(APIRequestError::RequestFailed(
                "Either path or file_id must be provided".to_string()
            ));
        }

        self.send_uploaded_file(
            chat_id,
            &final_file_id.unwrap(),
            text,
            chat_keypad,
            inline_keypad,
            disable_notification,
            reply_to_message_id,
            chat_keypad_type,
        ).await
    }

    pub async fn send_gif(
        &self,
        chat_id: &str,
        path: Option<&str>,
        file_id: Option<&str>,
        text: Option<&str>,
        file_name: Option<&str>,
        inline_keypad: Option<&Value>,
        chat_keypad: Option<&Value>,
        reply_to_message_id: Option<&str>,
        disable_notification: bool,
        chat_keypad_type: Option<&str>,
    ) -> Result<Value, APIRequestError> {
        let mut final_file_id = file_id.map(|s| s.to_string());

        if let Some(p) = path {
            let name = file_name.unwrap_or_else(|| {
                Path::new(p).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("animation.gif")
            });
            let upload_url = self.get_upload_url("Gif").await?;
            final_file_id = Some(self.upload_media_file(&upload_url, name, p).await?);
        }

        if final_file_id.is_none() {
            return Err(APIRequestError::RequestFailed(
                "Either path or file_id must be provided".to_string()
            ));
        }

        self.send_uploaded_file(
            chat_id,
            &final_file_id.unwrap(),
            text,
            chat_keypad,
            inline_keypad,
            disable_notification,
            reply_to_message_id,
            chat_keypad_type,
        ).await
    }

    pub async fn get_updates(
        &self,
        offset_id: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Value, APIRequestError> {
        let mut data = json!({});
        if let Some(oid) = offset_id {
            data["offset_id"] = json!(oid);
        }
        if let Some(l) = limit {
            data["limit"] = json!(l);
        }
        self.post("getUpdates", &data).await
    }

    pub async fn forward_message(
        &self,
        from_chat_id: &str,
        message_id: &str,
        to_chat_id: &str,
        disable_notification: bool,
    ) -> Result<Value, APIRequestError> {
        self.post("forwardMessage", &json!({
            "from_chat_id": from_chat_id,
            "message_id": message_id,
            "to_chat_id": to_chat_id,
            "disable_notification": disable_notification
        })).await
    }

    pub async fn edit_message_text(
        &self,
        chat_id: &str,
        message_id: &str,
        text: &str,
    ) -> Result<Value, APIRequestError> {
        self.post("editMessageText", &json!({
            "chat_id": chat_id,
            "message_id": message_id,
            "text": text
        })).await
    }

    pub async fn edit_inline_keypad(
        &self,
        chat_id: &str,
        message_id: &str,
        inline_keypad: &Value,
    ) -> Result<Value, APIRequestError> {
        self.post("editMessageKeypad", &json!({
            "chat_id": chat_id,
            "message_id": message_id,
            "inline_keypad": inline_keypad
        })).await
    }

    pub async fn delete_message(
        &self,
        chat_id: &str,
        message_id: &str,
    ) -> Result<Value, APIRequestError> {
        self.post("deleteMessage", &json!({
            "chat_id": chat_id,
            "message_id": message_id
        })).await
    }

    pub async fn set_commands(
        &self,
        bot_commands: &[Value],
    ) -> Result<Value, APIRequestError> {
        self.post("setCommands", &json!({"bot_commands": bot_commands})).await
    }

    pub async fn update_bot_endpoint(
        &self,
        url: &str,
        type_: &str,
    ) -> Result<Value, APIRequestError> {
        self.post("updateBotEndpoints", &json!({
            "url": url,
            "type": type_
        })).await
    }

    pub async fn remove_keypad(&self, chat_id: &str) -> Result<Value, APIRequestError> {
        self.post("editChatKeypad", &json!({
            "chat_id": chat_id,
            "chat_keypad_type": "Removed"
        })).await
    }

    pub async fn edit_chat_keypad(
        &self,
        chat_id: &str,
        chat_keypad: &Value,
    ) -> Result<Value, APIRequestError> {
        self.post("editChatKeypad", &json!({
            "chat_id": chat_id,
            "chat_keypad_type": "New",
            "chat_keypad": chat_keypad
        })).await
    }

    pub async fn get_name(&self, chat_id: &str) -> String {
        match self.get_chat(chat_id).await {
            Ok(chat) => {
                if let Some(chat_info) = chat.get("data").and_then(|d| d.get("chat")) {
                    let first_name = chat_info.get("first_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let last_name = chat_info.get("last_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    if !first_name.is_empty() && !last_name.is_empty() {
                        format!("{} {}", first_name, last_name)
                    } else if !first_name.is_empty() {
                        first_name.to_string()
                    } else if !last_name.is_empty() {
                        last_name.to_string()
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            }
            Err(_) => "Unknown".to_string(),
        }
    }

    pub async fn get_username(&self, chat_id: &str) -> String {
        match self.get_chat(chat_id).await {
            Ok(chat) => {
                chat.get("data")
                    .and_then(|d| d.get("chat"))
                    .and_then(|c| c.get("username"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("None")
                    .to_string()
            }
            Err(_) => "None".to_string(),
        }
    }

    pub async fn run(&self) -> Result<(), APIRequestError> {
        println!("Bot started running...");
        let bot = Arc::new(self.clone());

        {
            let latest = self.get_updates(None, Some(100)).await?;
            if let Some(data) = latest.get("data") {
                if let Some(updates) = data.get("updates").and_then(|u| u.as_array()) {
                    if updates.last().is_some() {
                        if let Some(next_offset) = data.get("next_offset_id").and_then(|v| v.as_str()) {
                            *self.offset_id.write() = Some(next_offset.to_string());
                            println!("Offset initialized to: {}", next_offset);
                        }
                    }
                }
            }
        }

        loop {
            let offset = self.offset_id.read().clone();
            let updates = self.get_updates(offset.as_deref(), Some(100)).await?;
            
            if let Some(data) = updates.get("data") {
                if let Some(updates_array) = data.get("updates").and_then(|u| u.as_array()) {
                    for update in updates_array {
                        let bot_clone = bot.clone();
                        let update_clone = update.clone();
                        tokio::spawn(async move {
                            bot_clone.process_update(&update_clone, bot_clone.clone()).await;
                        });
                    }
                    
                    if let Some(next_offset) = data.get("next_offset_id").and_then(|v| v.as_str()) {
                        *self.offset_id.write() = Some(next_offset.to_string());
                    }
                }
            }

            sleep(Duration::from_millis(100)).await;
        }
    }
}

impl std::fmt::Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Robot")
            .field("token", &format!("{}***", &self.token[..8.min(self.token.len())]))
            .field("timeout", &self.timeout)
            .field("platform", &self.platform)
            .field("message_handlers_count", &self.message_handlers.read().len())
            .field("callback_handlers_count", &self.callback_handlers.read().len())
            .field("has_inline_query_handler", &self.inline_query_handler.read().is_some())
            .finish()
    }
}

impl Clone for Robot {
    fn clone(&self) -> Self {
        Robot {
            token: self.token.clone(),
            timeout: self.timeout,
            auth: self.auth.clone(),
            session_name: self.session_name.clone(),
            key: self.key.clone(),
            platform: self.platform.clone(),
            offset_id: Arc::clone(&self.offset_id),
            client: self.client.clone(),
            sessions: Arc::clone(&self.sessions),
            message_handlers: Arc::clone(&self.message_handlers),
            callback_handlers: Arc::clone(&self.callback_handlers),
            inline_query_handler: Arc::clone(&self.inline_query_handler),
        }
    }
}

