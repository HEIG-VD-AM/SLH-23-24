pub const HTTP_PORT: u16 = 8090;

// Duration for the access token
pub const ACCESS_TOKEN_DURATION : usize = 60 * 15; // 15 minutes

// Duration for the refresh token
pub const REFRESH_TOKEN_DURATION: usize = 3600 * 24 * 7; // 7 day

// Duration for the verify link sent to user by email
pub const VERIFY_LINK_DURATION: usize = 30 * 60; // 10 minutes

// Regex for email validation
pub const MAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;

// ZXCVBN offers 5 levels of password strength. This is the level qualified as safely unguessable
pub const ZXCVBN_THRESHOLD: u8 = 3;

// Minimum length for a password
pub const MIN_PASSWORD_LENGTH : usize = 8;

// Maximum length for a password
pub const MAX_PASSWORD_LENGTH : usize = 64;

