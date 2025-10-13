#![feature(hash_map_macro)]

mod components;

use std::cmp::Ordering;

use crate::components::{
    buttons::{button, *},
    tags::*,
};
use jiff::civil::Date;
use leptos::{
    ev,
    html::{label, *},
    prelude::*,
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app);
}

fn app() -> impl IntoView {
    let tags = vec![
        TagData::Text {
            id: "commercial".into(),
            value: "Commercial Experience".into(),
        },
        TagData::Text {
            id: "personal".into(),
            value: "Personal Project".into(),
        },
    ];

    let project_data = vec![
        ProjectData {
            name: "Node4".into(),
            description: include_str!("../assets/content/node4.md").into(),
            image_url: None,
            start_date: Date::new(2023, 02, 01).unwrap(),
            end_date: None,
            tags: vec!["commercial".into()],
            links: vec![],
        },
        ProjectData {
            name: "C# LDM RSS Feed".into(),
            description: include_str!("../assets/content/csharplang_ldm_feed.md").into(),
            image_url: Some("images/csharplang_ldm_feed.png".into()),
            start_date: Date::new(2025, 09, 13).unwrap(),
            end_date: None,
            tags: vec!["personal".into()],
            links: vec![Link {
                url: "https://github.com/alasdair-cooper/csharplang-ldm-feed".into(),
                icon_name: Some("github".into()),
                text: "GitHub".into(),
            }],
        },
        ProjectData {
            name: "Password Generator".into(),
            description: include_str!("../assets/content/password_generator.md").into(),
            image_url: None,
            start_date: Date::new(2024, 11, 23).unwrap(),
            end_date: Some(Date::new(2024, 12, 07).unwrap()),
            tags: vec!["personal".into()],
            links: vec![Link {
                url: "https://github.com/alasdair-cooper/password-generator".into(),
                icon_name: Some("github".into()),
                text: "GitHub".into(),
            }],
        },
        ProjectData {
            name: "ebi Portfolios".into(),
            description: include_str!("../assets/content/ebi.md").into(),
            image_url: Some("images/ebi.png".into()),
            start_date: Date::new(2023, 02, 01).unwrap(),
            end_date: Some(Date::new(2025, 05, 01).unwrap()),
            tags: vec!["commercial".into()],
            links: vec![],
        },
    ];

    let mut projects = vec![];

    for project in project_data {
        projects.push(project.to_project(&tags));
    }

    projects.sort_by(|a, b| a.cmp_dates(b));

    vec![
        theme_picker().into_any(),
        header().into_any(),
        intro().into_any(),
        content(&projects).into_any(),
        footer().into_any(),
    ]
}

fn header() -> impl IntoView {
    leptos::html::header()
        .child(img().src("images/me.png"))
        .child(
            hgroup()
                .child(h1().child("Alasdair Cooper"))
                .child(p().child(".NET Web Developer")),
        )
}

fn theme_picker() -> impl IntoView {
    let is_current_theme = |val: Option<&str>| {
        window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item("theme").ok().flatten())
            .as_deref()
            == val
    };

    let clear_local_storage = || {
        window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.remove_item("theme").ok())
            .unwrap_or_default();
    };

    let persist_to_local_storage = move |val: &str| {
        window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.set_item("theme", val).ok())
            .unwrap_or_default();
    };

    span()
        .class("theme-picker")
        .child(
            label()
                .child(
                    input()
                        .attr("type", "radio")
                        .name("theme")
                        .on(ev::click, move |_| clear_local_storage())
                        .checked(is_current_theme(None)),
                )
                .child("Auto"),
        )
        .child(
            label()
                .child(
                    input()
                        .attr("type", "radio")
                        .name("theme")
                        .attr("data-theme", "light")
                        .on(ev::click, move |_| persist_to_local_storage("light"))
                        .checked(is_current_theme(Some("light"))),
                )
                .child("Light"),
        )
        .child(
            label()
                .child(
                    input()
                        .attr("type", "radio")
                        .name("theme")
                        .attr("data-theme", "dark")
                        .on(ev::click, move |_| persist_to_local_storage("dark"))
                        .checked(is_current_theme(Some("dark"))),
                )
                .child("Dark"),
        )
}

fn intro() -> impl IntoView {
    section()
        .class("intro")
        .child(h2().child("About Me"))
        .child(div().inner_html(markdown::to_html(include_str!(
            "../assets/content/about_me.md"
        ))))
}

fn content(projects: &Vec<Project>) -> impl IntoView {
    section()
        .class("projects")
        .child(h2().child("My Work"))
        // .child(alert(
        //     AlertLevel::Info,
        //     "Click on a tag or technology to filter by it.",
        // ))
        .child(projects.iter().rev().map(|x| project(x)).collect_view())
}

fn project(project: &Project) -> impl IntoView {
    article()
        .child(project.image_url.as_ref().map(|x| img().src(x)))
        .child(
            div().class("tags").child(
                project
                    .tags
                    .iter()
                    .filter_map(|x| match x {
                        data @ TagData::Text { id: _, value: _ } => Some(tag(data)),
                        _ => None,
                    })
                    .collect_view(),
            ),
        )
        .child(
            hgroup()
                .child(h2().child(project.name.clone()))
                .child(span().child(if let Some(end_date) = project.end_date {
                    format!(
                        "{} - {}",
                        project.start_date.strftime("%B %Y"),
                        end_date.strftime("%B %Y")
                    )
                } else {
                    format!("{} - Present", project.start_date.strftime("%B %Y"))
                })),
        )
        .child(div().inner_html(markdown::to_html(&project.description)))
        .child(
            div().class("links").child(
                project
                    .links
                    .iter()
                    .map(|x| {
                        button(
                            ButtonContent {
                                icon: x.icon_name.as_ref().map(|x| {
                                    Icon::Svg(
                                        components::icons::ICON_MAP
                                            .get(x.as_str())
                                            .expect(format!("icon '{}' not found", x).as_str()),
                                    )
                                }),
                                text: x.text.clone(),
                            },
                            ButtonEffect::Link { url: x.url.clone() },
                        )
                    })
                    .collect_view(),
            ),
        )
}

fn footer() -> impl IntoView {
    leptos::html::footer().inner_html(markdown::to_html(include_str!(
        "../assets/content/footer.md"
    )))
}

#[derive(Clone)]
struct Link {
    url: String,
    icon_name: Option<String>,
    text: String,
}

struct ProjectData {
    name: String,
    description: String,
    image_url: Option<String>,
    tags: Vec<String>,
    links: Vec<Link>,
    start_date: Date,
    end_date: Option<Date>,
}

impl ProjectData {
    fn to_project(&self, tags: &Vec<TagData>) -> Project {
        let mut tag_lookup = hash_map! {};

        for tag in tags {
            tag_lookup.insert(tag.id().clone(), tag.clone());
        }

        Project {
            name: self.name.clone(),
            description: self.description.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            image_url: self.image_url.clone(),
            links: self.links.clone(),
            tags: self
                .tags
                .iter()
                .map(|x| tag_lookup.get(x).expect("tag not found"))
                .map(|x| x.clone())
                .collect(),
        }
    }
}

struct Project {
    name: String,
    description: String,
    image_url: Option<String>,
    tags: Vec<TagData>,
    links: Vec<Link>,
    start_date: Date,
    end_date: Option<Date>,
}

impl Project {
    fn cmp_dates(&self, other: &Self) -> Ordering {
        match (self.end_date, other.end_date) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => self.start_date.cmp(&other.start_date),
        }
    }
}
