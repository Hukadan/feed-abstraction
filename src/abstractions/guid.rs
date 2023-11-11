use rss::Guid as RssGuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Default for Guid {
    // permalink is true by defaul for RssGuid
    fn default() -> Self {
        Guid {
            value: Default::default(),
            permalink: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn new_guid() -> Guid {
        Guid {
            value: "Value".into(),
            permalink: false,
        }
    }
    pub fn new_rss_guid() -> RssGuid {
        RssGuid {
            value: "Value".into(),
            permalink: false,
        }
    }
    #[test]
    fn default_abstract_to_rss_equal() {
        let guid1: RssGuid = Guid::default().into();
        let guid2 = RssGuid::default();
        assert_eq!(guid1, guid2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let guid1: Guid = RssGuid::default().into();
        let guid2 = Guid::default();
        assert_eq!(guid1, guid2);
    }
}
