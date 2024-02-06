use anyhow::anyhow;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use crate::consts::{ACCESS_TOKEN_DURATION, REFRESH_TOKEN_DURATION};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubClaims {
    email: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    nbf: usize,
    sub: String,
    role: Role,
}

pub fn create<T: Into<String>>(payload: T, role: Role) -> anyhow::Result<String> {
    // Get the current timestamp in seconds
    let current_time : usize = jsonwebtoken::get_current_timestamp() as usize;

    // Calculate the expiration time based on the token role
    let expiration_time : usize = match role {
        Role::Access => current_time + ACCESS_TOKEN_DURATION,
        Role::Refresh => current_time + REFRESH_TOKEN_DURATION,
    };

    // Get the secret key based on the token role
    let secret : String = match role {
        Role::Access => std::env::var("JWT_SECRET_ACCESS")?,
        Role::Refresh => std::env::var("JWT_SECRET_REFRESH")?
    };

    // Create the JWT header
    let header = Header::default();

    // Create the claims for the JWT
    let claims = Claims {
        exp: expiration_time,
        iat: current_time,
        nbf: current_time,
        sub: payload.into(),
        role,
    };

    // Encode the JWT with the header, claims, and secret key
    let jwt : String = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    return Ok(jwt);
}

/// Verify the validity of a JWT accordingly to its role (access or refresh)
/// Return the email contained in the JWT if it's valid
/// Return an error if the JWT is invalid
pub fn verify<T: Into<String>>(jwt: T, role: Role) -> anyhow::Result<String> {
    // Get the secret key based on the token role
    let secret = match role {
        Role::Access => std::env::var("JWT_SECRET_ACCESS")?,
        Role::Refresh => std::env::var("JWT_SECRET_REFRESH")?,
    };

    // Create validation rules for JWT
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    // Convert the token into a string
    let token = jwt.into();

    // Attempt to decode and validate the JWT
    let token_decoding_result = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation);

    match token_decoding_result {
        Ok(claims) if claims.claims.role == role => Ok(claims.claims.sub),
        Err(err) => {
            match *err.kind() {
                ErrorKind::InvalidToken => Err(anyhow!("Invalid token")),
                ErrorKind::ExpiredSignature => Err(anyhow!("Expired signature")),
                ErrorKind::ImmatureSignature => Err(anyhow!("Token not valid yet")),
                _ => Err(anyhow!("Unknown error")),
            }
        }
        _ => Err(anyhow!("Unknown error")),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::env;

    #[rstest(
    input,
    role_create,
    role_verify,
    expected,
    case("unit@test.com", Role::Access, Role::Access, true),
    case("unit@test.com", Role::Refresh, Role::Refresh, true),
    case("unit@test.com", Role::Access, Role::Refresh, false),
    case("unit@test.com", Role::Refresh, Role::Access, false),
    )]
    pub fn create_jwt_access_test(input: &str, role_create: Role, role_verify: Role, expected: bool) {
        env::set_var("JWT_SECRET_ACCESS", "dummy_access_var");
        env::set_var("JWT_SECRET_REFRESH", "dummy_refresh_var");
        let result = create(input, role_create).map(|token| verify(token, role_verify));
        let output = result.map(|res| res.is_ok()).unwrap_or(false);
        assert_eq!(output, expected);
    }

    #[rstest]
    pub fn token_invalid_test() {
        env::set_var("JWT_SECRET_ACCESS", "dummy_access_var");
        let mut token = create("user@test.com", Role::Access).unwrap();
        token.push_str("invalid");
        let result = verify(token, Role::Access);
        assert!(matches!(result, Err(anyhow::Error { .. })));
    }

    #[rstest]
    pub fn token_with_wrong_secret_test() {
        env::set_var("JWT_SECRET_ACCESS", "dummy_access_var");
        let token = create("user@test.com", Role::Access).unwrap();
        env::set_var("JWT_SECRET_ACCESS", "wrong_secret");
        let result = verify(token, Role::Access);
        assert!(matches!(result, Err(anyhow::Error { .. })));
    }

    #[rstest]
    pub fn token_exp_invalid_test() {

        let current_time = jsonwebtoken::get_current_timestamp() as usize;

        let claims = Claims {
            exp: current_time - 500,
            iat: current_time,
            nbf: current_time,
            sub: "user@test.com".into(),
            role: Role::Access,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("dummy_access_var".as_ref())).unwrap();
        let result = verify(token, Role::Access);
        assert!(matches!(result, Err(anyhow::Error { .. })));
    }

    #[rstest]
    pub fn token_nbf_invalid_test() {

        let current_time = jsonwebtoken::get_current_timestamp() as usize;
        let nbf_time = current_time + 500;

        let claims = Claims {
            exp: current_time + 600,
            iat: current_time,
            nbf: nbf_time,
            sub: "user@test.com".into(),
            role: Role::Access,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("dummy_access_var".as_ref())).unwrap();
        let result = verify(token, Role::Access);
        assert!(matches!(result, Err(anyhow::Error { .. })));
    }
}


