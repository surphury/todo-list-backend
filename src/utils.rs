use actix_web::http::header::HeaderValue;

use crate::jwt::verify_token;

pub fn validate_token(authorization: Option<&HeaderValue>) -> Result<i32, String> {
    match authorization {
        Some(token) => {
            let token = verify_token(token.to_str().unwrap());

            match token {
                Ok(user_token) => Ok(user_token.claims.id_user),
                Err(_) => Err(String::from("Invalid token")),
            }
        }
        None => Err(String::from("Verification failed")),
    }
}
