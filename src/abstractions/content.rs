use atom_syndication::Content as AtomContent;

#[derive(Clone, Debug, Default)]
pub struct Content {
    pub base: Option<String>,
    pub lang: Option<String>,
    pub value: Option<String>,
    pub src: Option<String>,
    pub content_type: Option<String>,
}

impl From<Content> for Option<String> {
    fn from(value: Content) -> Self {
        value.value
    }
}

impl From<Content> for String {
    fn from(value: Content) -> Self {
        match value.value {
            Some(text) => text,
            None => "".into(),
        }
    }
}

impl From<Option<String>> for Content {
    fn from(value: Option<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self {
            value: Some(value),
            ..Default::default()
        }
    }
}

impl From<AtomContent> for Content {
    fn from(value: AtomContent) -> Self {
        Self {
            base: value.base,
            lang: value.lang,
            value: value.value,
            src: value.src,
            content_type: value.content_type,
        }
    }
}

impl From<Content> for AtomContent {
    fn from(value: Content) -> Self {
        Self {
            base: value.base,
            lang: value.lang,
            value: value.value,
            src: value.src,
            content_type: value.content_type,
        }
    }
}
