use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        HashmapUserStore {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(user.email()) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email().to_string(), user);
                Ok(())
            }
        }   
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
     }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut user_store = HashmapUserStore::new();
        let user = User::new("email".to_string(), "password".to_string(), true);
        let result = user_store.add_user(user);
        assert_eq!(result, Ok(()));
    }
    #[test]
    fn test_get_user () {
        let mut user_store = HashmapUserStore::new();
        let user = User::new("email".to_string(), "password".to_string(), true);
        let user1 = user.clone();
        let _ = user_store.add_user(user);
        let result = user_store.get_user("email");
        assert_eq!(result, Ok(user1));
    }
    #[test]
    fn test_validate_user() {
        let mut user_store = HashmapUserStore::new();
        let user = User::new("email".to_string(), "password".to_string(), true);
        let _ = user_store.add_user(user);
        let email = "email";
        let password = "password";
        let result = user_store.validate_user(email, password);
        assert_eq!(result, Ok(()));
    }
}
