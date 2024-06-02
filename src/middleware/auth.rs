// Require Claims

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: u64,
    iat: u64,
}

impl Claims {
    pub fn new() -> Self {
        Self {
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + 86400,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    header: Header,
    payload: Claims,
    // don't feel good about sig type
    signature: String,
}

const SECRET: &str = "secret";

pub fn create_auth_token() -> String {
    let claims = Claims::new();

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap();

    println!("token: {:?}", token);
    token
}

pub fn validate_auth_token(token: String) {
    let res = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret("SECRET.as_ref()".as_bytes()),
        &Validation::default(),
    )
    .unwrap();

    println!("{:?}", res);
}

// impl Claims {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
