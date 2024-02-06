use zxcvbn::zxcvbn;
use crate::utils::consts::{COMMENT_MAX_LENGTH, COMMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH, ESTABLISHMENT_MIN_LENGTH, MAX_GRADE_REVIEW, MAX_PASSWORD_LENGTH, MIN_GRADE_REVIEW, MIN_PASSWORD_LENGTH, USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH, ZXCVBN_THRESHOLD};

pub fn is_password_valid(username: &str, password : &str) -> bool {
    if password.len() < MIN_PASSWORD_LENGTH || password.len() > MAX_PASSWORD_LENGTH {
        return false;
    }
    let estimate = zxcvbn(&password, &[username]).unwrap();
    return estimate.score() >= ZXCVBN_THRESHOLD
}

pub fn is_grade_valid(grade : u8) -> bool {
    grade >= MIN_GRADE_REVIEW && grade <= MAX_GRADE_REVIEW
}

pub fn is_comment_length_valid(input: &str) -> bool {
    input.len() >= COMMENT_MIN_LENGTH && input.len() <= COMMENT_MAX_LENGTH
}

pub fn is_username_length_valid(input: &str) -> bool {
    input.len() >= USERNAME_MIN_LENGTH && input.len() <= USERNAME_MAX_LENGTH
}

pub fn is_establishment_length_valid(input: &str) -> bool {
    input.len() >= ESTABLISHMENT_MIN_LENGTH && input.len() <= ESTABLISHMENT_MAX_LENGTH
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
    username,
    password,
    expected,
    case("username", "zxcvbn", false),
    case("username", "qwER43@!", false),
    case("username", "Tr0ub4dour&3", false),
    case("username", "correcthorsebatterystaple", true),
    case("username", "coRrecth0rseba++ery9.23.2007staple$", true),
    case("username", "", false),
    case("username", "pässwörd", false),
    case("username", "a b c 1 2 3", true),
    case("Tr0ub4dour&3", "Tr0ub4dour&3", false),
    ::trace
    )]
    pub fn password_validation_test(username: String, password: String, expected: bool) {
        assert_eq!(is_password_valid(&username, &password), expected);
    }

    #[rstest(
    input,
    expected,
    case(0, false),
    case(1, true),
    case(2, true),
    case(3, true),
    case(4, true),
    case(5, true),
    case(6, false),
    ::trace
    )]
    pub fn grade_validation_test(input: u8, expected: bool) {
        assert_eq!(is_grade_valid(input), expected);
    }

    #[rstest(
    input,
    expected,
    case("", false),
    case("a", true),
    case("a".repeat(COMMENT_MAX_LENGTH), true),
    case("a".repeat(COMMENT_MAX_LENGTH+1), false),
    ::trace
    )]
    pub fn comment_validation_test(input: String, expected: bool) {
        assert_eq!(is_comment_length_valid(&input), expected);
    }

    #[rstest(
    input,
    expected,
    case("", false),
    case("a", true),
    case("a".repeat(USERNAME_MAX_LENGTH), true),
    case("a".repeat(USERNAME_MAX_LENGTH+1), false),
    ::trace
    )]
    pub fn username_validation_test(input: String, expected: bool) {
        assert_eq!(is_username_length_valid(&input), expected);
    }

    #[rstest(
    input,
    expected,
    case("", false),
    case("a", true),
    case("a".repeat(ESTABLISHMENT_MAX_LENGTH), true),
    case("a".repeat(ESTABLISHMENT_MAX_LENGTH+1), false),
    ::trace
    )]
    pub fn establishment_validation_test(input: String, expected: bool) {
        assert_eq!(is_establishment_length_valid(&input), expected);
    }
}