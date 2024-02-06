use axum::Json;
use crate::backend::models::{NewUser, UserLogin, Token};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use log::{debug, info, trace};
use serde_json::json;
use time::{Duration, OffsetDateTime};
use tower_sessions::Session;
use uuid::Uuid;
use crate::{database, HBS};
use crate::backend::middlewares::AccessUser;
use axum::extract::Path;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use crate::database::email::Email;
use crate::consts::{VERIFY_LINK_DURATION};
use crate::email::{get_verification_url, send_mail};
use crate::utils::{jwt};
use crate::utils::crypto::{default_hash, hash_password, verify_password};
use crate::utils::input_val::{is_email_valid, is_password_valid};

pub async fn register(Json(user): Json<NewUser>) -> axum::response::Result<StatusCode> {
    info!("Register new user");

    // TODO: Register a new user
    // TODO: Send confirmation email
    // send_mail(...).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Normalize email by trimming and converting to lowercase
    let email : String = user.email.trim().to_ascii_lowercase();

    // Check if passwords match, email is valid, and password meets criteria
    if user.password != user.password2 ||
        !is_email_valid(&email) ||
        !is_password_valid(&user.password) {

        return Err(StatusCode::BAD_REQUEST.into());
    }

    // Hash the user's password
    let user_hash = hash_password(&user.password).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Check if the email already exists in the database
    match database::user::exists(&email) {
        Ok(true) => return Err(StatusCode::BAD_REQUEST.into()),
        Ok(false) => database::user::create(&email, &user_hash).or(Err(StatusCode::BAD_REQUEST))?,
        Err(_) => return Err(StatusCode::BAD_REQUEST.into()),
    };

    // Generate a unique verification token
    let uuid : String = Uuid::new_v4().to_string();

    // Add the token to the database with a expiration duration
    database::token::add(&email, &uuid, core::time::Duration::from_secs(VERIFY_LINK_DURATION as u64)).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Create a verification link for the email
    let subject : String = "Confirm your account".to_string();
    let link : String = get_verification_url(&uuid);
    let body : String = format!("Click on the following link to verify your account : {}", link);

    // Send the confirmation email
    send_mail(&email, &subject, &body).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(StatusCode::OK)
}

pub async fn verify(Path(token): Path<String>) -> Redirect {
    info!("Verify account");

    // TODO: Flag user's account as verified (with the given token)
    match database::token::consume(token) {
        Ok(email) => {
            match database::user::verify(&email) {
                // Redirect to a success page if verification is successful
                Ok(true) => Redirect::to("/?verify=ok"),
                // Redirect to a failure page if verification fails
                _ => Redirect::to("/?verify=failed"),
            }
        },
        // Redirect to a failure page if the token is invalid or expired
        _ => Redirect::to("/?verify=failed"),
    }
}

pub async fn login(Json(user_login): Json<UserLogin>) -> axum::response::Result<Json<Token>> {
    info!("Login user");

    // TODO: Login user
    // TODO: Generate refresh JWT

    // Normalize email by trimming and converting to lowercase
    let email : String = user_login.email.trim().to_ascii_lowercase();

    // Check if the user exists and the password matches
    // Note : Using get and .verified (instead of .exists, .get, and .verify) prevents calling the database three times
    let ok : bool = match database::user::get(&email) {
        Some(user) => {
            user.verified && verify_password(&user_login.password, &user.hash)
        },
        // If the user doesn't exist, use a default hash to prevent timing attacks
        None => {
            verify_password(&user_login.password, &default_hash());
            false
        }
    };

    match ok {
        true => {
            // Generate a refresh JWT token for the user
            let jwt: String = jwt::create(&email, jwt::Role::Refresh).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
            let token: Token = Token { token: jwt };
            Ok(Json::from(token))
        },
        false => Err(axum::response::IntoResponse::into_response((StatusCode::UNAUTHORIZED, "Login failed")).into()),
    }
}


/// Serve index page
/// If the user is logged, add a anti-CSRF token to the password change form
pub async fn home(
    session: Session,
    user: Option<AccessUser>,
) -> axum::response::Result<impl IntoResponse> {
    trace!("Serving home");

    // Create anti-CSRF token if the user is logged
    let infos = match user {
        Some(user) => {
            debug!("Add anti-CSRF token to home");

            // Generate anti-CSRF token
            let token = Uuid::new_v4().to_string();
            let expiration = OffsetDateTime::now_utc() + Duration::minutes(10);

            // Add token+exp to session
            session.insert("csrf", token.clone()).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
            session.insert("csrf_expiration", expiration.unix_timestamp()).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

            Some(json!({"email": user.email, "token": token}))
        },
        None => None, // Can't use user.map, async move are experimental
    };

    Ok(Html(HBS.render("index", &infos).unwrap()))
}
/// DEBUG/ADMIN endpoint
/// List pending emails to send
pub async fn email(Path(email): Path<String>) -> axum::response::Result<Json<Vec<Email>>> {
    match database::email::get(&email) {
        Ok(emails) => Ok(Json(emails)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    }
}
pub async fn logout(jar: CookieJar) -> (CookieJar, Redirect) {
    let jar = jar.remove(Cookie::from("access"));
    (jar, Redirect::to("/"))
}
pub async fn login_page() -> impl IntoResponse {
    Html(HBS.render("login", &Some(())).unwrap())
}

