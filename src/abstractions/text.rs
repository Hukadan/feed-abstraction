use atom_syndication::{Text as AtomText, TextType as AtomTextType};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub fn new_text_type() -> TextType {
        TextType::Html
    }
    pub fn new_atom_text_type() -> AtomTextType {
        AtomTextType::Html
    }
    pub fn new_text() -> Text {
        Text {
            value: "Value".into(),
            base: Some("Base".into()),
            lang: Some("Lang".into()),
            text_type: TextType::Html,
        }
    }
    pub fn new_atom_text() -> AtomText {
        AtomText {
            value: "Value".into(),
            base: Some("Base".into()),
            lang: Some("Lang".into()),
            r#type: AtomTextType::Html,
        }
    }
    #[test]
    fn default_abstract_to_atom_equal_text_type() {
        let texttype1: AtomTextType = TextType::default().into();
        let texttype2 = AtomTextType::default();
        assert_eq!(texttype1, texttype2);
    }
    #[test]
    fn default_atom_to_abstract_equal_text_type() {
        let texttype1: TextType = AtomTextType::default().into();
        let texttype2 = TextType::default();
        assert_eq!(texttype1, texttype2);
    }
    #[test]
    fn abstract_to_atom_equal_text_type() {
        let texttype1: AtomTextType = new_text_type().into();
        let texttype2 = new_atom_text_type();
        assert_eq!(texttype1, texttype2);
    }
    #[test]
    fn atom_to_abstract_equal_text_type() {
        let texttype1: TextType = new_atom_text_type().into();
        let texttype2 = new_text_type();
        assert_eq!(texttype1, texttype2);
    }
    #[test]
    fn default_abstract_to_atom_equal_text() {
        let text1: AtomText = Text::default().into();
        let text2 = AtomText::default();
        assert_eq!(text1, text2);
    }
    #[test]
    fn default_atom_to_abstract_equal_text() {
        let text1: Text = AtomText::default().into();
        let text2 = Text::default();
        assert_eq!(text1, text2);
    }
    #[test]
    fn abstract_to_atom_equal_text() {
        let text1: AtomText = new_text().into();
        let text2 = new_atom_text();
        assert_eq!(text1, text2);
    }
    #[test]
    fn atom_to_abstract_equal_text() {
        let text1: Text = new_atom_text().into();
        let text2 = new_text();
        assert_eq!(text1, text2);
    }
    #[test]
    fn atom_no_loss() {
        let text1: AtomText = new_atom_text();
        let text2 = text1.clone();
        let text2: Text = text2.into();
        let text2: AtomText = text2.into();
        assert_eq!(text1, text2)
    }
    #[test]
    fn abstract_to_string_equal() {
        let per: String = new_text().into();
        assert_eq!(per, String::from("Value"));
    }
    #[test]
    fn string_to_abstact_equal() {
        let text1 = Text {
            value: String::from("Value"),
            ..Default::default()
        };
        let text2: Text = String::from("Value").into();
        assert_eq!(text1, text2);
    }
}
