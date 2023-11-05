use super::{category::Category, generator::Generator, link::Link, person::Person, text::Text};
use atom_syndication::{FixedDateTime, Source as AtomSource, Text as AtomText};
use rss::Source as RssSource;

#[derive(Clone, Debug, Default)]
pub struct Source {
    pub title: Option<Text>,
    pub id: Option<String>,
    pub updated: Option<String>,
    pub authors: Vec<Person>,
    pub categories: Vec<Category>,
    pub contributors: Vec<Person>,
    pub generator: Option<Generator>,
    pub icon: Option<String>,
    pub links: Vec<Link>,
    pub rights: Option<Text>,
    pub subtitle: Option<Text>,
    pub logo: Option<String>,
}

impl From<RssSource> for Source {
    fn from(value: RssSource) -> Self {
        let links: Vec<Link> = vec![value.url.into()];
        let title: Option<Text> = value.title.map(|s| s.into());
        Self {
            title,
            links,
            ..Default::default()
        }
    }
}

impl From<Source> for RssSource {
    fn from(value: Source) -> Self {
        let url: String = if value.links.is_empty() {
            "".into()
        } else {
            value.links[0].clone().into()
        };
        Self {
            url,
            title: value.title.map(|s| s.into()),
        }
    }
}

impl From<AtomSource> for Source {
    fn from(value: AtomSource) -> Self {
        Self {
            title: Some(value.title.into()),
            id: Some(value.id),
            updated: Some(value.updated.to_string()),
            authors: value.authors.into_iter().map(|s| s.into()).collect(),
            categories: value.categories.into_iter().map(|s| s.into()).collect(),
            contributors: value.contributors.into_iter().map(|s| s.into()).collect(),
            generator: None,
            icon: value.icon,
            links: value.links.into_iter().map(|s| s.into()).collect(),
            rights: value.rights.map(|s| s.into()),
            subtitle: value.subtitle.map(|s| s.into()),
            logo: value.logo,
        }
    }
}

impl From<Source> for AtomSource {
    fn from(value: Source) -> Self {
        let title: AtomText = if let Some(text) = value.title {
            text.into()
        } else {
            AtomText::default()
        };
        let id: String = if let Some(text) = value.id {
            text
        } else {
            "".into()
        };
        let updated: FixedDateTime = if let Some(s) = value.updated {
            if let Ok(d) = FixedDateTime::parse_from_rfc3339(&s) {
                d
            } else {
                FixedDateTime::default()
            }
        } else {
            FixedDateTime::default()
        };
        Self {
            title,
            id,
            updated,
            authors: value.authors.into_iter().map(|s| s.into()).collect(),
            categories: value.categories.into_iter().map(|s| s.into()).collect(),
            contributors: value.contributors.into_iter().map(|s| s.into()).collect(),
            generator: value.generator.map(|s| s.into()),
            icon: value.icon,
            links: value.links.into_iter().map(|s| s.into()).collect(),
            logo: value.logo,
            rights: value.rights.map(|s| s.into()),
            subtitle: value.subtitle.map(|s| s.into()),
        }
    }
}
