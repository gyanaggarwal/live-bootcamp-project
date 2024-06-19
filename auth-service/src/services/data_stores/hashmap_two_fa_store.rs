use std::collections::HashMap;

use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    two_fa_codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_two_fa_code(&mut self, 
        email: &Email, 
        login_attempt_id: LoginAttemptId, 
        two_fa_code: TwoFACode) -> 
        Result<(), TwoFACodeStoreError> {
        self.two_fa_codes.insert(email.clone(), (login_attempt_id, two_fa_code));
        Ok(())
    }

    async fn get_two_fa_code(&self, email: &Email) ->
        Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.two_fa_codes.get(email) {
            Some(tcode) => Ok(tcode.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)   
        }
    }  

    async fn delete_two_fa_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.two_fa_codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }
}