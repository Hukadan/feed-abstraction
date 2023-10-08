use atom_syndication::Person;

#[derive(Clone, Default, Debug)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub uri: Option<String>,
}

impl From<String> for Author {
    fn from(value: String) -> Self {
        Self {
            name: value,
            ..Default::default()
        }
    }
}

impl From<Author> for String {
    fn from(value: Author) -> Self {
        match value.email {
            Some(email) => format!("{} ({})", email, value.name),
            None => value.name,
        }
    }
}

impl From<Person> for Author {
    fn from(value: Person) -> Self {
        Self {
            name: value.name,
            email: value.email,
            uri: value.uri,
        }
    }
}

impl From<Author> for Person {
    fn from(value: Author) -> Self {
        Self {
            name: value.name,
            email: value.email,
            uri: value.uri,
        }
    }
}
