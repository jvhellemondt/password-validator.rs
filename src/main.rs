fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum PasswordValidationErrors {
    TooShort,
    TooLong,
    MissingDigit,
    MissingUppercase,
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
            (
                !value.chars().any(|c| c.is_digit(10)),
                PasswordValidationErrors::MissingDigit,
            ),
            (
                !value.chars().any(|c| c.is_uppercase()),
                PasswordValidationErrors::MissingUppercase,
            ),
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
    fn valid_passwords() {
        let valid_passwords: Vec<&str> = vec![
            "Password1",
            "Pass1234",
            "Abcdef1",
            "HelloWorld123",
            "Secure9",
        ];
        for input in valid_passwords {
            let result = PasswordValidator::new(input);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Password(input.to_string()));
        }
    }

    #[test]
    fn invalid_passwords() {
        type TestCase = (&'static str, PasswordValidationErrors);
        let invalid_passwords: Vec<TestCase> = vec![
            ("short", PasswordValidationErrors::TooShort),
            (
                "this_password_is_way_too_long",
                PasswordValidationErrors::TooLong,
            ),
            ("password", PasswordValidationErrors::MissingDigit),
            ("password1", PasswordValidationErrors::MissingUppercase),
        ];
        for (input, expected_err) in invalid_passwords {
            let result = PasswordValidator::new(input);
            dbg!(&input, &result, &expected_err);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected_err);
        }
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
    fn it_should_not_allow_john_as_a_password_because_it_is_too_short_and_lacks_digits_and_uppercase_letters(
    ) {
        // TODO: Implement the logic for rejecting "john" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_supersecurepassword_as_a_password_because_it_exceeds_length_limit_and_lacks_digits(
    ) {
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
    fn it_should_not_allow_passwordpassword_as_a_password_because_it_lacks_digits_and_uppercase_letters(
    ) {
        // TODO: Implement the logic for rejecting "passwordpassword" as a password.
    }

    #[test]
    #[ignore]
    fn it_should_not_allow_1234567890_as_a_password_because_it_lacks_uppercase_letters() {
        // TODO: Implement the logic for rejecting "1234567890" as a password.
    }
}
