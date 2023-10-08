use atom_syndication::Generator as AtomGenerator;

#[derive(Clone, Debug, Default)]
pub struct Generator {
    pub value: String,
    pub uri: Option<String>,
    pub version: Option<String>,
}

impl From<AtomGenerator> for Generator {
    fn from(value: AtomGenerator) -> Self {
        Self {
            value: value.value,
            uri: value.uri,
            version: value.version,
        }
    }
}

impl From<Generator> for AtomGenerator {
    fn from(value: Generator) -> Self {
        Self {
            value: value.value,
            uri: value.uri,
            version: value.version,
        }
    }
}
