use serde_json::{json, Value};

pub struct ChatKeypadBuilder {
    rows: Vec<Value>,
}

impl ChatKeypadBuilder {
    pub fn new() -> Self {
        ChatKeypadBuilder { rows: Vec::new() }
    }

    pub fn row(mut self, buttons: &[Value]) -> Self {
        self.rows.push(json!({"buttons": buttons}));
        self
    }

    pub fn button(&self, id: &str, text: &str, type_: Option<&str>) -> Value {
        json!({
            "id": id,
            "type": type_.unwrap_or("Simple"),
            "button_text": text
        })
    }

    pub fn build(self, resize_keyboard: Option<bool>, on_time_keyboard: Option<bool>) -> Value {
        json!({
            "rows": self.rows,
            "resize_keyboard": resize_keyboard.unwrap_or(true),
            "on_time_keyboard": on_time_keyboard.unwrap_or(false)
        })
    }
}

impl Default for ChatKeypadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

