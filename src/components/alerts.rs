use leptos::{html::*, prelude::*};

pub enum _AlertLevel {
    Info,
}

impl _AlertLevel {
    pub fn _class(&self) -> String {
        match self {
            _AlertLevel::Info => "info".into(),
        }
    }
}

pub fn _alert(level: _AlertLevel, text: impl Into<String>) -> impl IntoView {
    div()
        .class(format!("alert {}", level._class()))
        .child(text.into())
}
