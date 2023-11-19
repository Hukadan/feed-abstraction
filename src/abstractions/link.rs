use atom_syndication::Link as AtomLink;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub href: String,
    pub rel: String,
    pub href_lang: Option<String>,
    pub mime_type: Option<String>,
    pub title: Option<String>,
    pub length: Option<String>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: Default::default(),
            // see https://www.rfc-editor.org/rfc/rfc4287#page-21
            rel: "alternate".to_string(),
            href_lang: Default::default(),
            mime_type: Default::default(),
            title: Default::default(),
            length: Default::default(),
        }
    }
}

impl From<String> for Link {
    fn from(value: String) -> Self {
        Self {
            href: value,
            ..Default::default()
        }
    }
}

impl From<Link> for String {
    fn from(value: Link) -> Self {
        value.href
    }
}

impl From<AtomLink> for Link {
    fn from(value: AtomLink) -> Self {
        Self {
            href: value.href,
            rel: value.rel,
            href_lang: value.hreflang,
            mime_type: value.mime_type,
            title: value.title,
            length: value.length,
        }
    }
}

impl From<Link> for AtomLink {
    fn from(value: Link) -> Self {
        Self {
            href: value.href,
            rel: value.rel,
            hreflang: value.href_lang,
            mime_type: value.mime_type,
            title: value.title,
            length: value.length,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub fn new_link() -> Link {
        Link {
            href: "Href".into(),
            rel: "Rel".into(),
            href_lang: Some("Href lang".into()),
            mime_type: Some("Mime type".into()),
            title: Some("Title".into()),
            length: Some("Length".into()),
        }
    }
    pub fn new_atom_link() -> AtomLink {
        AtomLink {
            href: "Href".into(),
            rel: "Rel".into(),
            hreflang: Some("Href lang".into()),
            mime_type: Some("Mime type".into()),
            title: Some("Title".into()),
            length: Some("Length".into()),
        }
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let link1: AtomLink = Link::default().into();
        let link2 = AtomLink::default();
        assert_eq!(link1, link2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let link1: Link = AtomLink::default().into();
        let link2 = Link::default();
        assert_eq!(link1, link2);
    }
    #[test]
    fn abstract_to_atom_equal() {
        let link1: AtomLink = new_link().into();
        let link2 = new_atom_link();
        assert_eq!(link1, link2);
    }
    #[test]
    fn atom_to_abstract_equal() {
        let link1: Link = new_atom_link().into();
        let link2 = new_link();
        assert_eq!(link1, link2);
    }

    #[test]
    fn abstract_to_string_equal() {
        let per: String = new_link().into();
        assert_eq!(per, String::from("Href"));
    }
    #[test]
    fn string_to_abstact_equal() {
        let link1 = Link {
            href: String::from("Href"),
            ..Default::default()
        };
        let link2: Link = String::from("Href").into();
        assert_eq!(link1, link2);
    }
}
