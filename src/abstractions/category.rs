use atom_syndication::Category as AtomCategory;
use rss::Category as RssCategory;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category {
    pub name: String,
    // domain for RSS
    pub scheme: Option<String>,
    pub label: Option<String>,
}

impl From<RssCategory> for Category {
    fn from(cat: RssCategory) -> Self {
        // Adding label based on name is only
        // interesting if name is not empty
        if cat.name.is_empty() {
            Self {
                name: cat.name.clone(),
                scheme: cat.domain,
                label: None,
            }
        } else {
            Self {
                name: cat.name.clone(),
                scheme: cat.domain,
                label: Some(cat.name),
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    pub fn new_category() -> Category {
        Category {
            name: "Category".into(),
            scheme: Some("Scheme".into()),
            label: Some("Label".into()),
        }
    }
    pub fn new_rss_category() -> RssCategory {
        RssCategory {
            name: "Category".into(),
            domain: Some("Scheme".into()),
        }
    }
    pub fn new_atom_category() -> AtomCategory {
        AtomCategory {
            term: "Category".into(),
            scheme: Some("Scheme".into()),
            label: Some("Label".into()),
        }
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let cat1: AtomCategory = Category::default().into();
        let cat2 = AtomCategory::default();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let cat1: Category = AtomCategory::default().into();
        let cat2 = Category::default();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn default_abstract_to_rss_equal() {
        let cat1: RssCategory = Category::default().into();
        let cat2 = RssCategory::default();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn default_rss_to_abstract_equal() {
        let cat1: Category = RssCategory::default().into();
        let cat2 = Category::default();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn abstract_to_atom_equal() {
        let cat1: AtomCategory = new_category().into();
        let cat2 = new_atom_category();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn atom_to_abstract_equal() {
        let cat1: Category = new_atom_category().into();
        let cat2 = new_category();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn abstract_to_rss_equal() {
        let cat1: RssCategory = new_category().into();
        let cat2 = new_rss_category();
        assert_eq!(cat1, cat2);
    }
    #[test]
    fn rss_to_abstract_equal() {
        let cat1: Category = new_rss_category().into();
        let mut cat2 = new_category();
        // RSS as no label. The label field is initialiase using
        // the name field
        cat2.label = Some(cat2.name.clone());
        assert_eq!(cat1, cat2);
    }
}
