use atom_syndication::{Text as AtomText, TextType as AtomTextType};

#[derive(Clone, Debug, Default)]
pub enum TextType {
    #[default]
    Text,
    Html,
    Xhtml,
}

impl From<AtomTextType> for TextType {
    fn from(tt: AtomTextType) -> Self {
        match tt {
            AtomTextType::Text => TextType::Text,
            AtomTextType::Html => TextType::Html,
            AtomTextType::Xhtml => TextType::Xhtml,
        }
    }
}

impl From<TextType> for AtomTextType {
    fn from(value: TextType) -> Self {
        match value {
            TextType::Text => AtomTextType::Text,
            TextType::Html => AtomTextType::Html,
            TextType::Xhtml => AtomTextType::Xhtml,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Text {
    pub value: String,
    pub base: Option<String>,
    pub lang: Option<String>,
    pub text_type: TextType,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self {
            value,
            base: None,
            lang: None,
            text_type: TextType::Text,
        }
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.value
    }
}

impl From<AtomText> for Text {
    fn from(value: AtomText) -> Self {
        Self {
            value: value.value,
            base: value.base,
            lang: value.lang,
            text_type: value.r#type.into(),
        }
    }
}

impl From<Text> for AtomText {
    fn from(value: Text) -> Self {
        Self {
            value: value.value,
            base: value.base,
            lang: value.lang,
            r#type: value.text_type.into(),
        }
    }
}
