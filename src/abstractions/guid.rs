use rss::Guid as RssGuid;

#[derive(Clone, Debug, Default)]
pub struct Guid {
    pub value: String,
    pub permalink: bool,
}

impl From<String> for Guid {
    fn from(id: String) -> Self {
        Self {
            value: id,
            permalink: false,
        }
    }
}

impl From<Guid> for String {
    fn from(value: Guid) -> Self {
        value.value
    }
}

impl From<RssGuid> for Guid {
    fn from(id: RssGuid) -> Self {
        Self {
            value: id.value,
            permalink: id.permalink,
        }
    }
}

impl From<Guid> for RssGuid {
    fn from(value: Guid) -> Self {
        Self {
            value: value.value,
            permalink: value.permalink,
        }
    }
}
