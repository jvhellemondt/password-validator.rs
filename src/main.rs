fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Clone)]
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
    fn new(value: &str) -> Result<Password, Vec<PasswordValidationErrors>> {
        let business_rules = [
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
        ];
        let errors: Vec<PasswordValidationErrors> = business_rules
            .iter()
            .filter_map(|&(condition, ref error)| condition.then(|| error.clone()))
            .collect();

        match errors.is_empty() {
            true => Ok(Password(value.to_string())),
            false => Err(errors),
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
        type TestCase = (&'static str, Vec<PasswordValidationErrors>);
        let invalid_passwords: Vec<TestCase> = vec![
            (
                "short",
                vec![
                    PasswordValidationErrors::TooShort,
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            (
                "this_password_is_way_too_long",
                vec![
                    PasswordValidationErrors::TooLong,
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            (
                "password",
                vec![
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            (
                "password1",
                vec![PasswordValidationErrors::MissingUppercase],
            ),
            (
                "passwordpassword",
                vec![
                    PasswordValidationErrors::TooLong,
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            (
                "john",
                vec![
                    PasswordValidationErrors::TooShort,
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            (
                "supersecurepassword",
                vec![
                    PasswordValidationErrors::TooLong,
                    PasswordValidationErrors::MissingDigit,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            ("admin123", vec![PasswordValidationErrors::MissingUppercase]),
            (
                "12345",
                vec![
                    PasswordValidationErrors::TooShort,
                    PasswordValidationErrors::MissingUppercase,
                ],
            ),
            ("Tiny1", vec![PasswordValidationErrors::TooShort]),
            (
                "PASSW",
                vec![
                    PasswordValidationErrors::TooShort,
                    PasswordValidationErrors::MissingDigit,
                ],
            ),
            (
                "1234567890",
                vec![PasswordValidationErrors::MissingUppercase],
            ),
            (
                "maxwell1_c",
                vec![PasswordValidationErrors::MissingUppercase],
            ),
            ("maxwellTheBe", vec![PasswordValidationErrors::MissingDigit]),
            (
                "thePhysical1234567",
                vec![PasswordValidationErrors::TooLong],
            ),
        ];
        for (input, expected_errs) in invalid_passwords {
            let result = PasswordValidator::new(input);
            dbg!(&input, &result, &expected_errs);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected_errs);
        }
    }
}
