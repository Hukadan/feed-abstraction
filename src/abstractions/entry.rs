use std::collections::BTreeMap;

use crate::abstractions::{
    category::Category,
    content::Content,
    enclosure::Enclosure,
    extension::{Extension, ExtensionMap},
    guid::Guid,
    link::Link,
    person::Person,
    source::Source,
    text::Text,
};
use atom_syndication::{
    extension::{Extension as AtomExtension, ExtensionMap as AtomExtensionMap},
    Category as AtomCategory, Entry as AtomEntry, Link as AtomLink, Person as AtomAuthor,
    Text as AtomText,
};
use chrono::{DateTime, FixedOffset};
use rss::{
    extension::dublincore::DublinCoreExtension,
    extension::itunes::ITunesItemExtension,
    extension::{Extension as RssExtension, ExtensionMap as RssExtensionMap},
    Category as RssCategory, Enclosure as RssEnclosure, Guid as RssGuid, Item as RssEntry,
};

#[derive(Clone, Debug, Default)]
pub struct Entry {
    pub title: Text,
    pub published: Option<DateTime<FixedOffset>>,
    pub updated: Option<DateTime<FixedOffset>>,
    pub guid: Guid,
    pub links: Vec<Link>,
    pub summary: Option<Text>,
    pub authors: Vec<Person>,
    pub feed_authors: Option<Vec<Person>>,
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub source: Option<Source>,
    pub content: Option<Content>,
    pub rights: Option<Text>,
    pub extensions: ExtensionMap,
    pub itunes_ext: Option<ITunesItemExtension>,
    pub dublin_core_ext: Option<DublinCoreExtension>,
}

impl From<RssEntry> for Entry {
    fn from(value: RssEntry) -> Self {
        let title: Text = match value.title {
            Some(text) => text.into(),
            None => Text::default(),
        };
        let guid: Guid = match value.guid {
            Some(id) => id.into(),
            None => Guid::default(),
        };
        let links: Vec<Link> = match value.link {
            Some(link) => {
                vec![link.into()]
            }
            None => vec![Link::default()],
        };
        let summary: Option<Text> = value.description.map(|s| s.into());
        let authors: Vec<Person> = match value.author {
            Some(text) => text.split("; ").map(|s| s.to_string().into()).collect(),
            None => vec![],
        };
        let categories: Vec<Category> = value.categories.into_iter().map(|s| s.into()).collect();
        let enclosure: Option<Enclosure> = value.enclosure.map(|s| s.into());
        let published: Option<DateTime<FixedOffset>> = match value.pub_date {
            Some(str_date) => match DateTime::<FixedOffset>::parse_from_rfc2822(&str_date) {
                Ok(re) => Some(re),
                _ => None,
            },
            None => None,
        };
        let extensions: ExtensionMap = {
            let mut bt1: BTreeMap<String, BTreeMap<String, Vec<Extension>>> = BTreeMap::new();
            for (key, value) in value.extensions.into_iter() {
                let mut bt2: BTreeMap<String, Vec<Extension>> = BTreeMap::new();
                for (key1, value1) in value.into_iter() {
                    let mut ve: Vec<Extension> = Vec::new();
                    for v in value1 {
                        ve.push(v.into());
                    }
                    bt2.insert(key1, ve);
                }
                bt1.insert(key, bt2);
            }
            bt1
        };
        Self {
            title,
            guid,
            links,
            summary,
            authors,
            // data not available in that case
            feed_authors: None,
            categories,
            comments: value.comments,
            enclosure,
            source: value.source.map(|s| s.into()),
            content: value.content.map(|s| s.into()),
            published,
            // no equivalent field on RSS item
            updated: None,
            rights: None,
            extensions,
            itunes_ext: value.itunes_ext,
            dublin_core_ext: value.dublin_core_ext,
        }
    }
}

impl From<Entry> for RssEntry {
    fn from(value: Entry) -> Self {
        let title: Option<String> = if value.title.value.is_empty() {
            None
        } else {
            Some(value.title.into())
        };
        let guid: Option<RssGuid> = if value.guid.value.is_empty() {
            None
        } else {
            Some(value.guid.into())
        };
        let link: Option<String> = if value.links.is_empty() {
            None
        } else {
            Some(value.links[0].clone().into())
        };
        let description: Option<String> = value.summary.map(|s| s.into());
        let author: Option<String> = if value.authors.is_empty() {
            None
        } else {
            let ast: Vec<String> = value.authors.into_iter().map(|s| s.into()).collect();
            Some(ast.join("; "))
        };
        let categories: Vec<RssCategory> = value.categories.into_iter().map(|s| s.into()).collect();
        let enclosure: Option<RssEnclosure> = value.enclosure.map(|s| s.into());
        let extensions: RssExtensionMap = {
            let mut bt1: BTreeMap<String, BTreeMap<String, Vec<RssExtension>>> = BTreeMap::new();
            for (key, value) in value.extensions.into_iter() {
                let mut bt2: BTreeMap<String, Vec<RssExtension>> = BTreeMap::new();
                for (key1, value1) in value.into_iter() {
                    let mut ve: Vec<RssExtension> = Vec::new();
                    for v in value1 {
                        ve.push(v.into());
                    }
                    bt2.insert(key1, ve);
                }
                bt1.insert(key, bt2);
            }
            bt1
        };
        Self {
            title,
            description,
            guid,
            link,
            author,
            categories,
            comments: value.comments,
            enclosure,
            content: value.content.map(|s| s.into()),
            source: value.source.map(|s| s.into()),
            pub_date: value.published.map(|s| s.to_rfc2822()),
            extensions,
            itunes_ext: value.itunes_ext,
            dublin_core_ext: value.dublin_core_ext,
        }
    }
}

impl From<AtomEntry> for Entry {
    fn from(entry: AtomEntry) -> Self {
        let links: Vec<Link> = entry.links.into_iter().map(|s| s.into()).collect();
        let authors: Vec<Person> = entry.contributors.into_iter().map(|s| s.into()).collect();
        let feed_authors: Option<Vec<Person>> = if entry.authors.is_empty() {
            None
        } else {
            Some(entry.authors.into_iter().map(|s| s.into()).collect())
        };
        let summary: Option<Text> = entry.summary.map(|s| s.into());
        let categories: Vec<Category> = entry.categories.into_iter().map(|s| s.into()).collect();
        let extensions: ExtensionMap = {
            let mut bt1: BTreeMap<String, BTreeMap<String, Vec<Extension>>> = BTreeMap::new();
            for (key, value) in entry.extensions.into_iter() {
                let mut bt2: BTreeMap<String, Vec<Extension>> = BTreeMap::new();
                for (key1, value1) in value.into_iter() {
                    let mut ve: Vec<Extension> = Vec::new();
                    for v in value1 {
                        ve.push(v.into());
                    }
                    bt2.insert(key1, ve);
                }
                bt1.insert(key, bt2);
            }
            bt1
        };
        Self {
            title: entry.title.into(),
            guid: entry.id.into(),
            links,
            summary,
            authors,
            feed_authors,
            categories,
            // Do not exist in Atom entry
            comments: None,
            enclosure: None,
            source: entry.source.map(|s| s.into()),
            content: entry.content.map(|s| s.into()),
            published: entry.published,
            updated: Some(entry.updated),
            rights: entry.rights.map(|s| s.into()),
            extensions,
            itunes_ext: None,
            dublin_core_ext: None,
        }
    }
}

impl From<Entry> for AtomEntry {
    fn from(value: Entry) -> Self {
        let links: Vec<AtomLink> = value.links.into_iter().map(|s| s.into()).collect();
        let authors: Vec<AtomAuthor> = if let Some(feed_authors) = value.feed_authors {
            feed_authors.into_iter().map(|s| s.into()).collect()
        } else {
            vec![]
        };
        let contributors: Vec<AtomAuthor> = value.authors.into_iter().map(|s| s.into()).collect();
        let summary: Option<AtomText> = value.summary.map(|s| s.into());
        let categories: Vec<AtomCategory> =
            value.categories.into_iter().map(|s| s.into()).collect();
        let updated = match value.updated {
            Some(d) => d,
            None => DateTime::<FixedOffset>::default(),
        };
        let extensions: AtomExtensionMap = {
            let mut bt1: BTreeMap<String, BTreeMap<String, Vec<AtomExtension>>> = BTreeMap::new();
            for (key, value) in value.extensions.into_iter() {
                let mut bt2: BTreeMap<String, Vec<AtomExtension>> = BTreeMap::new();
                for (key1, value1) in value.into_iter() {
                    let mut ve: Vec<AtomExtension> = Vec::new();
                    for v in value1 {
                        ve.push(v.into());
                    }
                    bt2.insert(key1, ve);
                }
                bt1.insert(key, bt2);
            }
            bt1
        };
        Self {
            title: value.title.into(),
            id: value.guid.into(),
            links,
            summary,
            authors,
            contributors,
            categories,
            source: value.source.map(|s| s.into()),
            content: value.content.map(|s| s.into()),
            published: value.published,
            updated,
            rights: value.rights.map(|s| s.into()),
            extensions,
        }
    }
}
