pub struct User {
    pub(crate) username: String
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            username: name
        }
    }
}