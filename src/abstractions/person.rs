use atom_syndication::Person as AtomPerson;

#[derive(Clone, Default, Debug)]
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
