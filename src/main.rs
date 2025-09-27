#![feature(hash_map_macro)]

use jiff::civil::Date;
use leptos::{html::*, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app);
}

fn app() -> impl IntoView {
    let tags = vec![
        Tag::Text {
            id: "commercial".into(),
            value: "Commercial Experience".into(),
        },
        Tag::Text {
            id: "personal".into(),
            value: "Personal Project".into(),
        },
    ];

    let project_data = vec![
        ProjectData {
            name: "C# LDM RSS Feed".into(),
            description: "".into(),
            image_url: "https://lipsum.app/640x480/".into(),
            start_date: Date::new(2025, 09, 13).unwrap(),
            end_date: None,
            tags: vec!["personal".into()],
            links: vec![Link {
                url: "https://github.com/alasdair-cooper/csharplang-ldm-feed".into(),
                icon_url: Some("images/github.svg".into()),
                text: "GitHub".into(),
            }],
        },
        ProjectData {
            name: "ebi Portfolios".into(),
            description: "".into(),
            image_url: "https://lipsum.app/640x480/".into(),
            start_date: Date::new(2025, 05, 01).unwrap(),
            end_date: Some(Date::new(2023, 02, 01).unwrap()),
            tags: vec!["commercial".into()],
            links: vec![],
        },
    ];

    let mut projects = vec![];

    for project in project_data {
        projects.push(project.to_project(&tags));
    }

    vec![
        header().into_any(),
        intro().into_any(),
        content(&projects).into_any(),
    ]
}

fn header() -> impl IntoView {
    leptos::html::header().child(
        hgroup()
            .child(h1().child("Alasdair Cooper"))
            .child(p().child(".NET Web Developer")),
    )
}

fn intro() -> impl IntoView {
    section()
        .child(p().child(
            "Hi, welcome to my website. My name is Alasdair and I am a .NET web developer based \
             in Walsall, UK. This site contains a breakdown of my professional roles and hobby \
             projects by technology used.",
        ))
        .child(h2().child("Using This Site"))
        .child(p().child("Click on a tag or technology to filter by it."))
}

fn content(projects: &Vec<Project>) -> impl IntoView {
    section()
        .class("projects")
        .child(projects.iter().map(|x| project(x)).collect_view())
}

fn project(project: &Project) -> impl IntoView {
    article()
        .child(img().src(project.image_url.clone()))
        .child(
            div().class("tags").child(
                project
                    .tags
                    .iter()
                    .filter_map(|x| match x {
                        Tag::Text { id: _, value } => Some(span().child(value.clone())),
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
        .child(project.description.clone())
        .child(
            div().class("links").child(
                project
                    .links
                    .iter()
                    .map(|x| {
                        a().href(x.url.clone()).target("_blank")
                            .child(if let Some(icon_url) = &x.icon_url {
                                Some(img().src(icon_url.clone()))
                            } else {
                                None
                            })
                            .child(span().child(x.text.clone()))
                    })
                    .collect_view(),
            ),
        )
}

#[derive(Clone)]
enum Tag {
    Icon { id: String, icon_url: String },
    Text { id: String, value: String },
}

impl Tag {
    fn id(&self) -> &String {
        match self {
            Tag::Icon { id, icon_url: _ } => id,
            Tag::Text { id, value: _ } => id,
        }
    }
}

#[derive(Clone)]
struct Link {
    url: String,
    icon_url: Option<String>,
    text: String,
}

struct ProjectData {
    name: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
    links: Vec<Link>,
    start_date: Date,
    end_date: Option<Date>,
}

impl ProjectData {
    fn to_project(&self, tags: &Vec<Tag>) -> Project {
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
    image_url: String,
    tags: Vec<Tag>,
    links: Vec<Link>,
    start_date: Date,
    end_date: Option<Date>,
}
