use phf::phf_map;

pub const GITHUB_ICON: &str = include_str!("../../assets/images/github.svg");

pub static ICON_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "github" => GITHUB_ICON,
};
