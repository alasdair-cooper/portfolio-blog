use leptos::{html::*, prelude::*};

pub enum AlertLevel {
    Info,
}

impl AlertLevel {
    pub fn class(&self) -> String {
        match self {
            AlertLevel::Info => "info".into(),
        }
    }
}

pub fn alert(level: AlertLevel, text: impl Into<String>) -> impl IntoView {
    div()
        .class(format!("alert {}", level.class()))
        .child(text.into())
}
