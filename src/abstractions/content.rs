use atom_syndication::Content as AtomContent;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[cfg(test)]
mod tests {
    use super::*;

    pub fn new_content() -> Content {
        Content {
            base: Some("Base".into()),
            lang: Some("Lang".into()),
            value: Some("Value".into()),
            src: Some("Source".into()),
            content_type: Some("Content type".into()),
        }
    }
    pub fn new_atom_content() -> AtomContent {
        AtomContent {
            base: Some("Base".into()),
            lang: Some("Lang".into()),
            value: Some("Value".into()),
            src: Some("Source".into()),
            content_type: Some("Content type".into()),
        }
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let cont1: AtomContent = Content::default().into();
        let cont2 = AtomContent::default();
        assert_eq!(cont1, cont2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let cont1: Content = AtomContent::default().into();
        let cont2 = Content::default();
        assert_eq!(cont1, cont2);
    }
    #[test]
    fn abstract_to_atom_equal() {
        let cont1: AtomContent = new_content().into();
        let cont2 = new_atom_content();
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn atom_to_abstract_equal() {
        let cont1: Content = new_atom_content().into();
        let cont2 = new_content();
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn atom_no_loss() {
        let cont1: AtomContent = new_atom_content();
        let cont2: Content = cont1.clone().into();
        let cont2: AtomContent = cont2.into();
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn abstract_to_string_equal() {
        let cont1: String = new_content().into();
        let cont2 = String::from("Value");
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn string_to_abstract_equal() {
        let cont1: Content = String::from("Value").into();
        let mut cont2 = Content::default();
        cont2.value = Some(String::from("Value"));
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn option_string_to_abstract_equal() {
        let cont1: Content = Some(String::from("Value")).into();
        let mut cont2 = Content::default();
        cont2.value = Some(String::from("Value"));
        assert_eq!(cont1, cont2)
    }
    #[test]
    fn option_string_none_to_abstract_equal() {
        let cont1: Content = None.into();
        let cont2 = Content::default();
        assert_eq!(cont1, cont2)
    }
}
