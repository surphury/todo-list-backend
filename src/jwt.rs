use super::model::{Claims, DBUser, Token};

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use std::env::var;

pub fn generate_token(user: &DBUser) -> Result<Token, Error> {
    let key = var("SECRET_KEY")
        .expect("SECRET_KEY must be set")
        .to_owned();

    let info_to_encode = Claims {
        id_user: user.id,
        exp: 1690155465,
    };

    let token = encode(
        &Header::default(),
        &info_to_encode,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;

    Ok(Token { token })
}

pub fn verify_token(recieved_token: &str) -> Result<TokenData<Claims>, Error> {
    let key = var("SECRET_KEY")
        .expect("SECRET_KEY must be set")
        .to_owned();
    let decoded_token = decode::<Claims>(
        recieved_token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    );

    decoded_token
}
