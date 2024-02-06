use axum::Json;
use http::StatusCode;
use log::info;
use tower_sessions::Session;
use crate::backend::middlewares::AccessUser;
use crate::backend::models::ChangePassword;
use crate::database;
use crate::utils::crypto::{hash_password, verify_password};
use crate::utils::input_val::is_password_valid;

pub async fn change_password (
    session: Session,
    user: AccessUser,
    Json(parameters): Json<ChangePassword>
) -> axum::response::Result<StatusCode> {
    info!("Changing user's password");

    // Check that the anti-CSRF token isn't expired
    let token_expiration = session.get::<i64>("csrf_expiration").or(Err(StatusCode::INTERNAL_SERVER_ERROR))?.ok_or(StatusCode::BAD_REQUEST)?;
    if token_expiration < time::OffsetDateTime::now_utc().unix_timestamp() {
        info!("Anti-CSRF token expired");
        Err((StatusCode::BAD_REQUEST, "Anti-CSRF token expired"))?;
    }

    // Compare the anti-CSRF token saved with the given one
    let token : String = session.get::<String>("csrf")
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(StatusCode::BAD_REQUEST)?;
    if token != parameters.csrf {
        info!("Anti-CSRF tokens don't match");
        Err((StatusCode::BAD_REQUEST, "Anti-CSRF tokens don't match"))?;
    }

    // TODO : Check the parameters then update the DB with the new password

    // Check if passwords match and the new password is not the same as the old one.
    if parameters.password != parameters.password2 || parameters.password == parameters.old_password {
        Err(StatusCode::BAD_REQUEST)?;
    }

    // Check if the new password meets validity criteria.
    if !is_password_valid(&parameters.password) {
        return Err(StatusCode::BAD_REQUEST.into());
    }

    let user_db = match database::user::get(&user.email) {
        Some(user) => user,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    };

    // Verify if the old password provided matches the stored password hash.
    if !verify_password(&parameters.old_password, &user_db.hash) {
        Err(StatusCode::BAD_REQUEST)?;
    }

    let user_hash : String = hash_password(&parameters.password).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Hash the new password before updating it in the database.
    database::user::change_password(&user.email, &user_hash).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::OK)
}
