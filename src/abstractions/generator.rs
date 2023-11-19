use atom_syndication::Generator as AtomGenerator;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub fn new_generator() -> Generator {
        Generator {
            value: "Value".into(),
            uri: Some("Uri".into()),
            version: Some("Version".into()),
        }
    }
    pub fn new_atom_generator() -> AtomGenerator {
        AtomGenerator {
            value: "Value".into(),
            uri: Some("Uri".into()),
            version: Some("Version".into()),
        }
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let gen1: AtomGenerator = Generator::default().into();
        let gen2 = AtomGenerator::default();
        assert_eq!(gen1, gen2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let gen1: Generator = AtomGenerator::default().into();
        let gen2 = Generator::default();
        assert_eq!(gen1, gen2);
    }
    #[test]
    fn abstract_to_atom_equal() {
        let gen1: AtomGenerator = new_generator().into();
        let gen2 = new_atom_generator();
        assert_eq!(gen1, gen2);
    }
    #[test]
    fn atom_to_abstract_equal() {
        let gen1: Generator = new_atom_generator().into();
        let gen2 = new_generator();
        assert_eq!(gen1, gen2);
    }
}
