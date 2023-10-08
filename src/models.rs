use atom_syndication::{Category as AtomCategory, Entry as AtomItem, Feed as AtomFeed};
use rss::{Category as RssCategory, Channel as RssFeed, Item as RssItem};
use std::error::Error;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
pub enum FeedType {
    Rss,
    Atom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeedUrl {
    Rss(String),
    Atom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feed {
    Atom(AtomFeed),
    Rss(RssFeed),
}

impl Feed {
    pub fn read_from<R: BufRead>(data: R, ftype: FeedType) -> Result<Feed, Box<dyn Error>> {
        match ftype {
            FeedType::Rss => {
                let feed = RssFeed::read_from(data)?;
                Ok(Feed::Rss(feed))
            }
            FeedType::Atom => {
                let feed = AtomFeed::read_from(data)?;
                Ok(Feed::Atom(feed))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Category {
    name: String,
    description: String,
}

impl From<RssCategory> for Category {
    fn from(item: RssCategory) -> Self {
        Self {
            name: item.name,
            description: "".to_string(),
        }
    }
}

impl From<AtomCategory> for Category {
    fn from(item: AtomCategory) -> Self {
        Self {
            name: item.term,
            description: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Article {
    uuid: String,
    date: String,
    title: String,
    categories: Vec<Category>,
    authors: Vec<String>,
    content: String,
    summary: String,
    link: String,
}

impl From<RssItem> for Article {
    fn from(item: RssItem) -> Self {
        let title = item.title().get_or_insert("").to_string();
        let date = item.pub_date().get_or_insert("").to_string();
        let content = item.content().get_or_insert("").to_string();
        let link = item.link().get_or_insert("").to_string();
        let mut categories: Vec<Category> = Vec::new();
        for c in item.categories {
            categories.push(c.into());
        }
        let mut authors: Vec<String> = Vec::new();
        if let Some(au) = item.author {
            authors.push(au);
        }
        Self {
            uuid: "a".to_string(),
            date,
            title,
            content,
            categories,
            authors,
            // no summary in rss
            // may be show the begining of content
            summary: "".to_string(),
            link,
        }
    }
}

impl From<AtomItem> for Article {
    fn from(item: AtomItem) -> Self {
        let mut content: String = String::from("");
        if let Some(con) = item.content {
            if let Some(value) = con.value() {
                content = value.to_string();
            }
        }
        let mut summary: String = String::from("");
        if let Some(text) = item.summary {
            summary = text.value;
        };
        let mut categories: Vec<Category> = Vec::new();
        for c in item.categories {
            categories.push(c.into());
        }
        Self {
            uuid: item.id.clone(),
            date: item.updated.to_string(),
            title: item.title.to_string(),
            categories,
            content,
            authors: vec![],
            summary,
            link: item.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ArticleFeed {
    name: String,
    articles: Vec<Article>,
    source: String,
}
