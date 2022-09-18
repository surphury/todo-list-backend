use actix_web::http::header::HeaderValue;

use crate::jwt::verify_token;
use crate::model::VerificationError;

pub fn validate_token(authorization: Option<&HeaderValue>) -> Result<i32, VerificationError> {
    match authorization {
        Some(token) => {
            let token = verify_token(token.to_str().unwrap());

            match token {
                Ok(user_token) => Ok(user_token.claims.id_user),
                Err(_) => Err(VerificationError::InvalidToken),
            }
        }
        None => Err(VerificationError::EmptyToken),
    }
}
