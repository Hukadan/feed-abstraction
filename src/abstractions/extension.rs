use atom_syndication::extension::Extension as AtomExtension;
use rss::extension::Extension as RssExtension;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Extension {
    pub name: String,
    pub value: Option<String>,
    pub attrs: BTreeMap<String, String>,
    pub children: BTreeMap<String, Vec<Extension>>,
}

impl From<Extension> for RssExtension {
    fn from(value: Extension) -> Self {
        let mut bt: BTreeMap<String, Vec<RssExtension>> = BTreeMap::new();
        for (key, value) in value.children.into_iter() {
            let mut ch_extensions: Vec<RssExtension> = Vec::new();
            for v in value {
                ch_extensions.push(v.into());
            }
            bt.insert(key, ch_extensions);
        }
        Self {
            name: value.name,
            value: value.value,
            attrs: value.attrs,
            children: bt,
        }
    }
}

impl From<RssExtension> for Extension {
    fn from(value: RssExtension) -> Self {
        let mut bt: BTreeMap<String, Vec<Extension>> = BTreeMap::new();
        for (key, value) in value.children.into_iter() {
            let mut ch_extensions: Vec<Extension> = Vec::new();
            for v in value {
                ch_extensions.push(v.into());
            }
            bt.insert(key, ch_extensions);
        }
        Self {
            name: value.name,
            value: value.value,
            attrs: value.attrs,
            children: bt,
        }
    }
}

impl From<Extension> for AtomExtension {
    fn from(value: Extension) -> Self {
        let mut bt: BTreeMap<String, Vec<AtomExtension>> = BTreeMap::new();
        for (key, value) in value.children.into_iter() {
            let mut ch_extensions: Vec<AtomExtension> = Vec::new();
            for v in value {
                ch_extensions.push(v.into());
            }
            bt.insert(key, ch_extensions);
        }
        Self {
            name: value.name,
            value: value.value,
            attrs: value.attrs,
            children: bt,
        }
    }
}

impl From<AtomExtension> for Extension {
    fn from(value: AtomExtension) -> Self {
        let mut bt: BTreeMap<String, Vec<Extension>> = BTreeMap::new();
        for (key, value) in value.children.into_iter() {
            let mut ch_extensions: Vec<Extension> = Vec::new();
            for v in value {
                ch_extensions.push(v.into());
            }
            bt.insert(key, ch_extensions);
        }
        Self {
            name: value.name,
            value: value.value,
            attrs: value.attrs,
            children: bt,
        }
    }
}

pub type ExtensionMap = BTreeMap<String, BTreeMap<String, Vec<Extension>>>;
