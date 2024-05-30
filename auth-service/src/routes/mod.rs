use serde::{Deserialize, Serialize};

mod login;
mod logout;
mod signup;
mod verify_2fa;
mod verify_token;

pub use login::*;
pub use logout::*;
pub use signup::*;
pub use verify_2fa::*;
pub use verify_token::*;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct RouteResponse {
    pub message: String,
}


