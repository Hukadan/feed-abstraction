use atom_syndication::Link as AtomLink;

#[derive(Clone, Debug)]
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
