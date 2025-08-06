use regex::Regex;

/// Validate a username against the following rules:
/// - Must contain between 5-20 characters.
/// - Must start with a letter.
/// - Allowed characters: letters, numbers, underscores
/// - No consecutive underscores.
/// # Arguments
/// - `username`: The username to validate
/// # Returns
/// `true` if username is valid.
pub fn validate_username(username: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z][\w\d]{4,19}$").unwrap();
    let unders = Regex::new(r"__").unwrap();
    re.is_match(username) && !unders.is_match(username)
}

/// Validate a password against the following rules:
/// - Must contain between 8 and 30 characters.
/// - Must contain one of each: capital letter, lowercase letter, number, symbol.
/// - Allowed characters: letters, numbers, symbols (`! @ # $ % ^ & * ? + =`)
/// # Arguments
/// - `username`: The password to validate
/// # Returns
/// `true` if the password is valid.
pub fn validate_password(password: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z\d!@#$%^&*?+=]{8,30}$").unwrap();
    let cap = Regex::new(r"[A-Z]").unwrap();
    let low = Regex::new(r"[a-z]").unwrap();
    let num = Regex::new(r"\d").unwrap();
    let sym = Regex::new(r"[!@#$%^&*?+=]").unwrap();
    re.is_match(password)
        && cap.is_match(password)
        && low.is_match(password)
        && num.is_match(password)
        && sym.is_match(password)
}

pub fn validate_email(email: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap()
        .is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_usernames() {
        assert!(validate_username("b1gd3vd0g"));
        assert!(validate_username("mr_robot"));
        assert!(validate_username("d_365"));
        assert!(validate_username("thegoodbadchadplayer"));
    }

    #[test]
    fn test_invalid_usernames() {
        assert!(!validate_username("pete"));
        assert!(!validate_username("thegoodbadchadplayer1"));
        assert!(!validate_username("mr________smithers"));
        assert!(!validate_username("24_7_stinky"));
        assert!(!validate_username("_stupid_hoe"));
        assert!(!validate_username("$tinky_girl"));
    }

    #[test]
    fn test_valid_passwords() {
        assert!(validate_password("p4$5w0Rd"));
        assert!(validate_password("Buffy!53"));
        assert!(validate_password("1234567890abcdefghijABCDEFGHI$"));
        assert!(validate_password("#redDOG77"));
        assert!(validate_password("J0EY&&phoebe"));
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(!validate_password(r"p4$5w0R\"));
        assert!(!validate_password("buffy!53"));
        assert!(!validate_password("1234567890abcdefghijABCDEFGHIJ$"));
        assert!(!validate_password("redDOG77"));
        assert!(!validate_password("JOEY&&phoebe"));
    }

    #[test]
    fn test_valid_emails() {
        assert!(validate_email("user@mail.com"));
        assert!(validate_email("user+mailbox@sub.domain.co.uk"));
        assert!(validate_email("user123@slither.io"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!validate_email("@mail.com"));
        assert!(!validate_email("user@.com"));
        assert!(!validate_email("user@mail"));
        assert!(!validate_email("user@mail."));
        assert!(!validate_email("user@mail.c"));
    }
}
