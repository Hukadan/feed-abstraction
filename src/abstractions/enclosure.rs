use rss::Enclosure as RssEnclosure;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enclosure {
    pub url: String,
    pub length: String,
    pub mime_type: String,
}

impl From<RssEnclosure> for Enclosure {
    fn from(value: RssEnclosure) -> Self {
        Self {
            url: value.url,
            length: value.length,
            mime_type: value.mime_type,
        }
    }
}

impl From<Enclosure> for RssEnclosure {
    fn from(value: Enclosure) -> Self {
        Self {
            url: value.url,
            length: value.length,
            mime_type: value.mime_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn new_enclosure() -> Enclosure {
        Enclosure {
            url: "https://enclosure.org".into(),
            length: "100".into(),
            mime_type: "text/html".into(),
        }
    }
    pub fn new_rss_enclosure() -> RssEnclosure {
        RssEnclosure {
            url: "https://enclosure.org".into(),
            length: "100".into(),
            mime_type: "text/html".into(),
        }
    }
    #[test]
    fn default_abstract_to_rss_equal() {
        let enc1: RssEnclosure = Enclosure::default().into();
        let enc2 = RssEnclosure::default();
        assert_eq!(enc1, enc2);
    }
    #[test]
    fn default_rss_to_abstract_equal() {
        let enc1: Enclosure = RssEnclosure::default().into();
        let enc2 = Enclosure::default();
        assert_eq!(enc1, enc2);
    }
    #[test]
    fn abstract_to_rss_equal() {
        let enc1: RssEnclosure = new_enclosure().into();
        let enc2 = new_rss_enclosure();
        assert_eq!(enc1, enc2);
    }
    #[test]
    fn rss_to_abstract_equal() {
        let enc1: Enclosure = new_rss_enclosure().into();
        let enc2 = new_enclosure();
        assert_eq!(enc1, enc2);
    }
}
