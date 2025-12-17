use serde_json::{json, Value};

pub struct InlineBuilder {
    rows: Vec<Value>,
}

impl InlineBuilder {
    pub fn new() -> Self {
        InlineBuilder { rows: Vec::new() }
    }

    pub fn row(mut self, buttons: &[Value]) -> Self {
        if buttons.is_empty() {
            panic!("At least one button must be provided to row");
        }
        self.rows.push(json!({"buttons": buttons}));
        self
    }

    pub fn button_simple(&self, id: &str, text: &str) -> Value {
        json!({
            "id": id,
            "type": "Simple",
            "button_text": text
        })
    }

    pub fn button_selection(&self, id: &str, text: &str, selection: &Value) -> Value {
        json!({
            "id": id,
            "type": "Selection",
            "button_text": text,
            "button_selection": selection
        })
    }

    pub fn button_calendar(
        &self,
        id: &str,
        title: &str,
        type_: &str,
        default_value: Option<&str>,
        min_year: Option<&str>,
        max_year: Option<&str>,
    ) -> Value {
        let mut calendar = json!({
            "title": title,
            "type": type_
        });
        
        if let Some(dv) = default_value {
            calendar["default_value"] = json!(dv);
        }
        if let Some(my) = min_year {
            calendar["min_year"] = json!(my);
        }
        if let Some(maxy) = max_year {
            calendar["max_year"] = json!(maxy);
        }

        json!({
            "id": id,
            "type": "Calendar",
            "button_text": title,
            "button_calendar": calendar
        })
    }

    pub fn button_number_picker(
        &self,
        id: &str,
        title: &str,
        min_value: &str,
        max_value: &str,
        default_value: Option<&str>,
    ) -> Value {
        let mut picker = json!({
            "title": title,
            "min_value": min_value,
            "max_value": max_value
        });
        
        if let Some(dv) = default_value {
            picker["default_value"] = json!(dv);
        }

        json!({
            "id": id,
            "type": "NumberPicker",
            "button_text": title,
            "button_number_picker": picker
        })
    }

    pub fn button_string_picker(
        &self,
        id: &str,
        title: Option<&str>,
        items: &[String],
        default_value: Option<&str>,
    ) -> Value {
        let mut picker = json!({
            "items": items
        });
        
        if let Some(dv) = default_value {
            picker["default_value"] = json!(dv);
        }
        if let Some(t) = title {
            picker["title"] = json!(t);
        }

        json!({
            "id": id,
            "type": "StringPicker",
            "button_text": title.unwrap_or("choice"),
            "button_string_picker": picker
        })
    }

    pub fn button_location(
        &self,
        id: &str,
        type_: &str,
        location_image_url: &str,
        default_pointer_location: Option<&Value>,
        default_map_location: Option<&Value>,
        title: Option<&str>,
    ) -> Value {
        let mut loc = json!({
            "type": type_,
            "location_image_url": location_image_url
        });
        
        if let Some(dpl) = default_pointer_location {
            loc["default_pointer_location"] = dpl.clone();
        }
        if let Some(dml) = default_map_location {
            loc["default_map_location"] = dml.clone();
        }
        if let Some(t) = title {
            loc["title"] = json!(t);
        }

        json!({
            "id": id,
            "type": "Location",
            "button_text": title.unwrap_or("location"),
            "button_location": loc
        })
    }

    pub fn button_textbox(
        &self,
        id: &str,
        title: Option<&str>,
        type_line: &str,
        type_keypad: &str,
        place_holder: Option<&str>,
        default_value: Option<&str>,
    ) -> Value {
        let mut textbox = json!({
            "type_line": type_line,
            "type_keypad": type_keypad
        });
        
        if let Some(ph) = place_holder {
            textbox["place_holder"] = json!(ph);
        }
        if let Some(dv) = default_value {
            textbox["default_value"] = json!(dv);
        }
        if let Some(t) = title {
            textbox["title"] = json!(t);
        }

        json!({
            "id": id,
            "type": "Textbox",
            "button_text": title.unwrap_or("Text"),
            "button_textbox": textbox
        })
    }

    pub fn button_payment(
        &self,
        id: &str,
        title: &str,
        amount: i32,
        description: Option<&str>,
    ) -> Value {
        let mut payment = json!({
            "title": title,
            "amount": amount
        });
        
        if let Some(d) = description {
            payment["description"] = json!(d);
        }

        json!({
            "id": id,
            "type": "Payment",
            "button_text": title,
            "button_payment": payment
        })
    }

    pub fn button_camera_image(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "CameraImage",
            "button_text": title
        })
    }

    pub fn button_camera_video(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "CameraVideo",
            "button_text": title
        })
    }

    pub fn button_gallery_image(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "GalleryImage",
            "button_text": title
        })
    }

    pub fn button_gallery_video(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "GalleryVideo",
            "button_text": title
        })
    }

    pub fn button_file(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "File",
            "button_text": title
        })
    }

    pub fn button_audio(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "Audio",
            "button_text": title
        })
    }

    pub fn button_record_audio(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "RecordAudio",
            "button_text": title
        })
    }

    pub fn button_my_phone_number(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "MyPhoneNumber",
            "button_text": title
        })
    }

    pub fn button_my_location(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "MyLocation",
            "button_text": title
        })
    }

    pub fn button_link(&self, id: &str, title: &str, url: &str) -> Value {
        json!({
            "id": id,
            "type": "Link",
            "button_text": title,
            "url": url
        })
    }

    pub fn button_ask_my_phone_number(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "AskMyPhoneNumber",
            "button_text": title
        })
    }

    pub fn button_ask_location(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "AskLocation",
            "button_text": title
        })
    }

    pub fn button_barcode(&self, id: &str, title: &str) -> Value {
        json!({
            "id": id,
            "type": "Barcode",
            "button_text": title
        })
    }

    pub fn build(self) -> Value {
        json!({"rows": self.rows})
    }
}

impl Default for InlineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

