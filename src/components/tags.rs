use leptos::{html::*, prelude::*};

#[derive(Clone)]
pub enum TagData {
    _Icon { id: String, icon_url: String },
    Text { id: String, value: String },
}

impl TagData {
    pub fn id(&self) -> &String {
        match self {
            TagData::_Icon { id, icon_url: _ } => id,
            TagData::Text { id, value: _ } => id,
        }
    }
}

pub fn tag(tag: &TagData) -> impl IntoView {
    match tag {
        TagData::_Icon { id: _, icon_url } => img().src(icon_url.clone()).into_any(),
        TagData::Text { id: _, value } => small().class("text-tag").child(value.clone()).into_any(),
    }
}
