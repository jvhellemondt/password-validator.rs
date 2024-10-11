fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum PasswordValidationErrors {
    TooShort,
    TooLong,
    MissingDigit
}

#[derive(Debug, PartialEq)]
struct Password(String);

#[derive(Debug)]
struct PasswordValidator;

impl PasswordValidator {
    fn new(value: &str) -> Result<Password, PasswordValidationErrors> {
        let error = [
            (value.len() <= 5, PasswordValidationErrors::TooShort),
            (value.len() >= 15, PasswordValidationErrors::TooLong),
            (!value.chars().any(|c| c.is_digit(10)), PasswordValidationErrors::MissingDigit),
        ]
            .into_iter()
            .find_map(|(condition, error)| condition.then_some(error));
        match error {
            Some(e) => Err(e),
            None => Ok(Password(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_able_to_create_a_password() {
        let value = "password1";
        let password_result = PasswordValidator::new(&value);
        assert!(password_result.is_ok());
        assert_eq!(password_result.unwrap(), Password(value.to_string()));
    }

    #[test]
    fn it_should_verify_passwords_are_longer_than_5_characters() {
        let value = "short";
        let password_result = PasswordValidator::new(&value);
        assert!(password_result.is_err());
        assert_eq!(password_result.unwrap_err(), PasswordValidationErrors::TooShort);
    }

    #[test]
    fn it_should_verify_passwords_are_less_than_15_characters() {
        let value = "this_password_is_way_too_long";
        let password_result = PasswordValidator::new(&value);
        assert!(password_result.is_err());
        assert_eq!(password_result.unwrap_err(), PasswordValidationErrors::TooLong);
    }

    #[test]
    fn it_should_verify_passwords_contain_at_least_1_digit() {
        let value = "password";
        let password_result = PasswordValidator::new(&value);
        assert!(password_result.is_err());
        assert_eq!(password_result.unwrap_err(), PasswordValidationErrors::MissingDigit);
    }

    #[test]
    #[ignore]
    fn it_should_verify_passwords_contain_at_least_1_uppercase_character() {
        // TODO: Implement the logic for rejecting passwords without at least one uppercase character.
    }

    #[test]
    #[ignore]
    fn it_should_allow_pass1234_as_a_password_because_it_meets_all_criteria() {
        // TODO: Implement the logic for accepting "Pass1234" as a valid password.
    }

    #[test]
    #[ignore]
    fn it_should_allow_abcdef1_as_a_password_because_it_is_valid() {
        // TODO: Implement the logic for accepting "Abcdef1" as a valid password.
    }

    #[test]
    #[ignore]
    fn it_should_allow_helloworld123_as_a_password_because_it_meets_all_criteria() {
        // TODO: Implement the logic for accepting "HelloWorld123" as a valid password.
    }

    #[test]
    #[ignore]
    fn it_should_allow_secure9_as_a_password_because_it_is_valid() {
        // TODO: Implement the logic for accepting "Secure9" as a valid password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_john_as_a_password_because_it_is_too_short_and_lacks_digits_and_uppercase_letters() {
        // TODO: Implement the logic for rejecting "john" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_supersecurepassword_as_a_password_because_it_exceeds_length_limit_and_lacks_digits() {
        // TODO: Implement the logic for rejecting "SuperSecurePassword" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_admin123_as_a_password_because_it_lacks_uppercase_letters() {
        // TODO: Implement the logic for rejecting "admin123" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_12345_as_a_password_because_it_lacks_uppercase_letters() {
        // TODO: Implement the logic for rejecting "12345" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_tiny1_as_a_password_because_it_is_too_short() {
        // TODO: Implement the logic for rejecting "Tiny1" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_passw_as_a_password_because_it_lacks_digits() {
        // TODO: Implement the logic for rejecting "PASSW" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_passwordpassword_as_a_password_because_it_lacks_digits_and_uppercase_letters() {
        // TODO: Implement the logic for rejecting "passwordpassword" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_1234567890_as_a_password_because_it_lacks_uppercase_letters() {
        // TODO: Implement the logic for rejecting "1234567890" as a password.
    }
}
