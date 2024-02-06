use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use log::info;
use crate::backend::middlewares::RefreshUser;
use crate::consts::ACCESS_TOKEN_DURATION;
use crate::utils::jwt;

pub async fn get_access(user: RefreshUser, jar: CookieJar) -> axum::response::Result<CookieJar> {
    info!("Get access JWT from refresh JWT");
    // User's refresh token is already checked through the extractor RefreshUser
    // You can trust the email given in the parameter "user"

    // TODO : Create access JWT for email in user
    let jwt : String = match jwt::create(&user.email, jwt::Role::Access) {
        Ok(jwt) => jwt,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR).into()),
    };


    // Add JWT to jar
    let cookie = Cookie::build(("access", jwt))
                              .path("/") // CHECK CA FAIT QUOI
                              .max_age(time::Duration::seconds(ACCESS_TOKEN_DURATION as i64))
                              .same_site(SameSite::Strict)
                              .http_only(true);
                              // .secure(true) add when HTTPS is enabled
    let jar = jar.add(cookie);

    Ok(jar)
}
