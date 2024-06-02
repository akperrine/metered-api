// Require Claims

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

// impl Claims {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
