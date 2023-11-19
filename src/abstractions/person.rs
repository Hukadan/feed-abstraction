use atom_syndication::Person as AtomPerson;

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
    pub uri: Option<String>,
}

impl From<String> for Person {
    fn from(value: String) -> Self {
        Self {
            name: value,
            ..Default::default()
        }
    }
}

impl From<Person> for String {
    fn from(value: Person) -> Self {
        match value.email {
            Some(email) => format!("{} ({})", email, value.name),
            None => value.name,
        }
    }
}

impl From<AtomPerson> for Person {
    fn from(value: AtomPerson) -> Self {
        Self {
            name: value.name,
            email: value.email,
            uri: value.uri,
        }
    }
}

impl From<Person> for AtomPerson {
    fn from(value: Person) -> Self {
        Self {
            name: value.name,
            email: value.email,
            uri: value.uri,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub fn new_person() -> Person {
        Person {
            name: String::from("John"),
            email: Some(String::from("john@lenon.com")),
            uri: Some(String::from("https//lenon.com")),
        }
    }
    pub fn new_atom_person() -> AtomPerson {
        AtomPerson {
            name: String::from("John"),
            email: Some(String::from("john@lenon.com")),
            uri: Some(String::from("https//lenon.com")),
        }
    }
    #[test]
    fn default_abstract_to_atom_equal() {
        let per1: AtomPerson = Person::default().into();
        let per2 = AtomPerson::default();
        assert_eq!(per1, per2);
    }
    #[test]
    fn default_atom_to_abstract_equal() {
        let per1: Person = AtomPerson::default().into();
        let per2 = Person::default();
        assert_eq!(per1, per2);
    }
    #[test]
    fn atom_no_loss() {
        let per1: AtomPerson = new_atom_person();
        let per2 = per1.clone();
        let per2: Person = per2.into();
        let per2: AtomPerson = per2.into();
        assert_eq!(per1, per2)
    }
    #[test]
    fn abstract_to_atom_equal() {
        let per1: AtomPerson = new_person().into();
        let per2 = new_atom_person();
        assert_eq!(per1, per2)
    }
    #[test]
    fn atom_to_abstract_equal() {
        let per1: Person = new_atom_person().into();
        let per2 = new_person();
        assert_eq!(per1, per2)
    }
    #[test]
    fn abstract_to_string_equal() {
        let per: String = new_person().into();
        assert_eq!(per, String::from("john@lenon.com (John)"));
    }
    #[test]
    fn string_to_abstact_equal() {
        let per1 = Person {
            name: String::from("john@lenon.com (John)"),
            ..Default::default()
        };
        let per2: Person = String::from("john@lenon.com (John)").into();
        assert_eq!(per1, per2);
    }
}
