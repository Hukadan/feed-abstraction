use atom_syndication::Category as AtomCategory;
use rss::Category as RssCategory;

#[derive(Clone, Debug, Default)]
pub struct Category {
    pub name: String,
    // domain for RSS
    pub scheme: Option<String>,
    pub label: Option<String>,
}

impl From<RssCategory> for Category {
    fn from(cat: RssCategory) -> Self {
        Self {
            name: cat.name.clone(),
            scheme: cat.domain,
            label: Some(cat.name),
        }
    }
}

impl From<Category> for RssCategory {
    fn from(value: Category) -> Self {
        Self {
            name: value.name,
            domain: value.scheme,
        }
    }
}

impl From<AtomCategory> for Category {
    fn from(cat: AtomCategory) -> Self {
        Self {
            name: cat.term,
            scheme: cat.scheme,
            label: cat.label,
        }
    }
}

impl From<Category> for AtomCategory {
    fn from(value: Category) -> Self {
        Self {
            term: value.name,
            scheme: value.scheme,
            label: value.label,
        }
    }
}
