#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn build_user(email: String, username: String) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}