use leptos::{ev, html::*, prelude::*};

pub struct ButtonContent {
    pub text: String,
    pub icon_url: Option<String>,
}

impl From<String> for ButtonContent {
    fn from(value: String) -> Self {
        Self {
            text: value,
            icon_url: None,
        }
    }
}

pub enum ButtonEffect {
    Link { url: String, target: String },
    _Action { action: Callback<()> },
}

pub fn button(content: ButtonContent, effect: ButtonEffect) -> impl IntoView {
    match effect {
        ButtonEffect::Link { url, target } => a()
            .class("button")
            .href(url)
            .target(target)
            .child(content.icon_url.map(|x| img().src(x)))
            .child(content.text)
            .into_any(),
        ButtonEffect::_Action { action } => leptos::html::button()
            .class("button")
            .child(content.icon_url.map(|x| img().src(x)))
            .child(content.text)
            .on(ev::click, move |_| action.run(()))
            .into_any(),
    }
}
