use super::{category::Category, generator::Generator, link::Link, person::Person, text::Text};
use atom_syndication::{FixedDateTime, Source as AtomSource, Text as AtomText};
use chrono::{DateTime, FixedOffset};
use rss::Source as RssSource;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source {
    pub title: Option<Text>,
    pub id: Option<String>,
    pub updated: Option<DateTime<FixedOffset>>,
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
        // If the url string is empty, generate
        // an empty vector
        let links: Vec<Link> = if value.url.is_empty() {
            vec![]
        } else {
            vec![value.url.into()]
        };
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
        // even if the links vector is empty we
        // have to provide a string
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

impl Default for Source {
    fn default() -> Self {
        Source {
            title: Option::default(),
            id: Option::default(),
            updated: Some(DateTime::<FixedOffset>::from_str("1970-01-01T00:00:00Z").unwrap()),
            authors: Vec::default(),
            categories: Vec::default(),
            contributors: Vec::default(),
            generator: Option::default(),
            icon: Option::default(),
            links: Vec::default(),
            rights: Option::default(),
            subtitle: Option::default(),
            logo: Option::default(),
        }
    }
}

impl From<AtomSource> for Source {
    fn from(value: AtomSource) -> Self {
        // Only need to store the value if it is
        // different from the default value
        let title: Option<Text> = if value.title == AtomText::default() {
            None
        } else {
            Some(value.title.into())
        };
        // Only need to store the value if it is
        // different from empty string
        let id: Option<String> = if value.id.is_empty() {
            None
        } else {
            Some(value.id.into())
        };
        Self {
            title,
            id,
            updated: Some(value.updated),
            authors: value.authors.into_iter().map(|s| s.into()).collect(),
            categories: value.categories.into_iter().map(|s| s.into()).collect(),
            contributors: value.contributors.into_iter().map(|s| s.into()).collect(),
            generator: value.generator.map(|s| s.into()),
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
            s
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

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    pub(crate) fn new_source() -> Source {
        Source {
            title: Some(crate::abstractions::text::tests::new_text()),
            id: Some("id".into()),
            updated: Some(DateTime::default()),
            authors: vec![crate::abstractions::person::tests::new_person()],
            categories: vec![crate::abstractions::category::tests::new_category()],
            contributors: vec![crate::abstractions::person::tests::new_person()],
            generator: Some(crate::abstractions::generator::tests::new_generator()),
            icon: Some("Icon".into()),
            links: vec![crate::abstractions::link::tests::new_link()],
            rights: Some(crate::abstractions::text::tests::new_text()),
            subtitle: Some(crate::abstractions::text::tests::new_text()),
            logo: Some("Logo".into()),
        }
    }

    pub(crate) fn new_source_for_rss() -> Source {
        Source {
            title: Some(Text {
                value: "Title".into(),
                ..Default::default()
            }),
            links: vec!["Href".to_string().into()],
            ..Default::default()
        }
    }

    pub(crate) fn new_atom_source() -> AtomSource {
        AtomSource {
            title: crate::abstractions::text::tests::new_atom_text(),
            id: "id".into(),
            updated: DateTime::default(),
            authors: vec![crate::abstractions::person::tests::new_atom_person()],
            categories: vec![crate::abstractions::category::tests::new_atom_category()],
            contributors: vec![crate::abstractions::person::tests::new_atom_person()],
            generator: Some(crate::abstractions::generator::tests::new_atom_generator()),
            icon: Some("Icon".into()),
            links: vec![crate::abstractions::link::tests::new_atom_link()],
            logo: Some("Logo".into()),
            rights: Some(crate::abstractions::text::tests::new_atom_text()),
            subtitle: Some(crate::abstractions::text::tests::new_atom_text()),
        }
    }

    pub(crate) fn new_rss_source() -> RssSource {
        RssSource {
            url: "Href".into(),
            title: Some("Title".into()),
        }
    }

    #[test]
    fn default_abstract_to_rss_equal() {
        let src1: RssSource = Source::default().into();
        let src2 = RssSource::default();
        assert_eq!(src1, src2);
    }
    #[test]
    fn default_rss_to_abstract_equal() {
        let src1: Source = RssSource::default().into();
        let src2 = Source::default();
        assert_eq!(src1, src2);
    }
    #[test]
    fn abstract_to_rss_equal() {
        let src1: RssSource = new_source_for_rss().into();
        let src2 = new_rss_source();
        assert_eq!(src1, src2);
    }
    // Rss source as only two fields. Check how to
    // test this properly
    #[test]
    fn rss_to_abstract_equal() {
        let src1: Source = new_rss_source().into();
        let src2 = new_source_for_rss();
        assert_eq!(src1, src2);
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let src1: AtomSource = Source::default().into();
        let src2 = AtomSource::default();
        assert_eq!(src1, src2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let src1: Source = AtomSource::default().into();
        let src2 = Source::default();
        assert_eq!(src1, src2);
    }
    #[test]
    fn abstract_to_atom_equal() {
        let src1: AtomSource = new_source().into();
        let src2 = new_atom_source();
        assert_eq!(src1, src2);
    }
    #[test]
    fn atom_to_abstract_equal() {
        let src1: Source = new_atom_source().into();
        let src2 = new_source();
        assert_eq!(src1, src2);
    }
}
