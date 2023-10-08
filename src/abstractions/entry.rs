use crate::abstractions::{
    author::Author, category::Category, enclosure::Enclosure, guid::Guid, link::Link, text::Text,
};
use atom_syndication::{
    Category as AtomCategory, Entry as AtomEntry, Link as AtomLink, Person as AtomAuthor,
    Text as AtomText,
};
use rss::{Category as RssCategory, Enclosure as RssEnclosure, Guid as RssGuid, Item as RssEntry};

#[derive(Clone, Debug, Default)]
pub struct Entry {
    pub title: Text,
    pub guid: Guid,
    pub links: Vec<Link>,
    pub summary: Option<Text>,
    pub authors: Vec<Author>,
    pub feed_authors: Option<Vec<Author>>,
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    pub enclosure: Option<Enclosure>,
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
        let authors: Vec<Author> = match value.author {
            Some(text) => text.split("; ").map(|s| s.to_string().into()).collect(),
            None => vec![],
        };
        let categories: Vec<Category> = value.categories.into_iter().map(|s| s.into()).collect();
        let enclosure: Option<Enclosure> = value.enclosure.map(|s| s.into());
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
        let author: Option<String> = if value.authors.is_empty() {
            None
        } else {
            let ast: Vec<String> = value.authors.into_iter().map(|s| s.into()).collect();
            Some(ast.join("; "))
        };
        let categories: Vec<RssCategory> = value.categories.into_iter().map(|s| s.into()).collect();
        let enclosure: Option<RssEnclosure> = value.enclosure.map(|s| s.into());
        Self {
            title,
            guid,
            link,
            author,
            categories,
            comments: value.comments,
            enclosure,
            ..Default::default()
        }
    }
}

impl From<AtomEntry> for Entry {
    fn from(entry: AtomEntry) -> Self {
        let links: Vec<Link> = entry.links.into_iter().map(|s| s.into()).collect();
        let authors: Vec<Author> = entry.contributors.into_iter().map(|s| s.into()).collect();
        let feed_authors: Option<Vec<Author>> = if entry.authors.is_empty() {
            None
        } else {
            Some(entry.authors.into_iter().map(|s| s.into()).collect())
        };
        let summary: Option<Text> = entry.summary.map(|s| s.into());
        let categories: Vec<Category> = entry.categories.into_iter().map(|s| s.into()).collect();
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
        Self {
            title: value.title.into(),
            id: value.guid.into(),
            links,
            summary,
            authors,
            contributors,
            categories,
            ..Default::default()
        }
    }
}
