use once_cell::sync::Lazy;
use regex::Regex;
use zxcvbn::zxcvbn;
use crate::consts::{MAIL_REGEX, MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH, ZXCVBN_THRESHOLD};

pub fn is_password_valid(password : &String) -> bool {
    if password.len() < MIN_PASSWORD_LENGTH || password.len() > MAX_PASSWORD_LENGTH {
        return false;
    }
    let estimate = zxcvbn(&password, &[]).unwrap();
    return estimate.score() >= ZXCVBN_THRESHOLD
}

pub fn is_email_valid(email : &String) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(MAIL_REGEX).unwrap());
    RE.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
    input,
    expected,
    case("zxcvbn", false),
    case("qwER43@!", false),
    case("Tr0ub4dour&3", false),
    case("correcthorsebatterystaple", true),
    case("coRrecth0rseba++ery9.23.2007staple$", true),
    case("", false),
    case("pässwörd", false),
    case("a b c 1 2 3", true),
    ::trace
    )]
    pub fn password_validation_test(input: String, expected: bool) {
        assert_eq!(is_password_valid(&input), expected);
    }

    #[rstest(
    input,
    expected,
    case("toto", false),
    case("filipe.fortunato@heig-vd.ch", true),
    case("fabio@disapproved.solutions", true),
    case("john@aol...com", false),
    case("unit@test", false),
    case("unit@test.", false),
    case("UNIT@TEST.COM", false),
    case("123@unit.test", true),
    case("", false),
    ::trace
    )]
    pub fn mail_regex_test(input: String, expected: bool) {
        assert_eq!(is_email_valid(&input), expected);
    }
}