use leptos::{ev, html::*, prelude::*};

pub enum Icon {
    _Url(String),
    Svg(&'static str),
}

pub struct ButtonContent {
    pub text: String,
    pub icon: Option<Icon>,
}

impl From<String> for ButtonContent {
    fn from(value: String) -> Self {
        Self {
            text: value,
            icon: None,
        }
    }
}

pub enum ButtonEffect {
    Link { url: String },
    _Action { action: Callback<()> },
}

pub fn button(content: ButtonContent, effect: ButtonEffect) -> impl IntoView {
    match effect {
        ButtonEffect::Link { url } => a()
            .class("button")
            .href(url)
            .child(content.icon.map(|x| match x {
                Icon::_Url(url) => img().class("icon").src(url).into_any(),
                Icon::Svg(svg) => span().class("icon").inner_html(svg).into_any(),
            }))
            .child(content.text)
            .into_any(),
        ButtonEffect::_Action { action } => leptos::html::button()
            .class("button")
            .child(content.icon.map(|x| match x {
                Icon::_Url(url) => img().class("icon").src(url).into_any(),
                Icon::Svg(svg) => span().class("icon").inner_html(svg).into_any(),
            }))
            .child(content.text)
            .on(ev::click, move |_| action.run(()))
            .into_any(),
    }
}
