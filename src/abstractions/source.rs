use super::{author::Author, category::Category, link::Link, text::Text};

#[derive(Clone, Debug, Default)]
pub struct Source {
    pub title: Text,
    pub id: String,
    pub updated: String,
    pub authors: Vec<Author>,
    pub categories: Vec<Category>,
    pub contributors: Vec<Author>,
    // pub generator
    pub icon: Option<String>,
    pub links: Vec<Link>,
    pub rights: Option<Text>,
    pub subtitle: Option<Text>,
}
