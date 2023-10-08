use rss::Enclosure as RssEnclosure;

#[derive(Clone, Debug, Default)]
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
