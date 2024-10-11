# password-validator.rs

## Problem Description

Write a function (or a stateless class) for validating passwords. Passwords must meet the following
criteria.
* Between 5 and 15 characters long
* Contains at least one digit
* Contains at least one upper case letter
* Return an object containing a boolean result and an errors key that — when provided with an invalid password — contains
* an error message or type for all errors in occurrence. There can be multiple errors at a single time.

## FA²STR 

### FIND
The password validator is ultimately responsible for validating passwords; it ensures all validation rules are met and returns a detailed list of validation failures when the password is invalid.

Business rules:

1. Check Length: Ensure the password is between 5 and 15 characters.
2. Check for Digits: Verify that the password contains at least one digit.
3. Check for Upper Case Letters: Ensure the password includes at least one uppercase letter.

### ARCHITECTURE

Value in: 
* An unknown value from json.

Value out (success):
```rust
struct Password(String);
```

Value out (errors):
* Vec with errors

```rust
pub enum PasswordValidationError {
  TooShort,
  TooLong,
  MissingDigit,
  MissingUppercase,
}
```

API:
```rust
let password_result = Password::new(input);
```

### AUTOMATE
Use cargo test in watch-mode with in-file tests.

### SPECIFY
- it_should_be_able_to_create_a_password
- it_should_verify_passwords_are_longer_than_5_characters
- it_should_verify_passwords_are_less_than_15_characters
- it_should_verify_passwords_contain_at_least_1_digit
- it_should_verify_passwords_contain_at_least_1_uppercase_character

- it_should_allow_Pass1234_as_a_password_because_it_meets_all_criteria
- it_should_allow_Abcdef1_as_a_password_because_it_is_valid
- it_should_allow_HelloWorld123_as_a_password_because_it_meets_all_criteria
- it_should_allow_Secure9_as_a_password_because_it_is_valid

- it_should_not_allow_john_as_a_password_because_it_is_too_short_and_lacks_digits_and_uppercase_letters
- it_should_not_allow_supersecurepassword_as_a_password_because_it_exceeds_length_limit_and_lacks_digits
- it_should_not_allow_admin123_as_a_password_because_it_lacks_uppercase_letters
- it_should_not_allow_12345_as_a_password_because_it_lacks_uppercase_letters
- it_should_not_allow_Tiny1_as_a_password_because_it_is_too_short
- it_should_not_allow_PASSW_as_a_password_because_it_lacks_digits
- it_should_not_allow_passwordpassword_as_a_password_because_it_lacks_digits_and_uppercase_letters
- it_should_not_allow_1234567890_as_a_password_because_it_lacks_uppercase_letters

### TEST ✅

### REFINE ✅

## Grading Checklist

- ✅ I have implemented the minimum requirements listed in the project description
- ✅ I have Programmed By Wishful Thinking, designing the response API before it was actually created
- ✅ I have Worked Backwards, starting at the Assert, then going to the Act and the Arrange
- ✅ I have tests that validate the following statements
  - ✅ "maxwell1_c" returns a false-y response because of a lack of uppercase characters
  - ✅ "maxwellTheBe" returns a false-y response because of a lack of digits
  - ✅ "thePhysical1234567" returns a false-y response because of exceeding the 15 character length
- ✅ Once I have made the aforementioned tests pass, I have refactored my test specifications to use it.each() to perform
  parameterization if there is sufficient duplication (see Tip #3 here)
- ✅ There is no duplication in my test code or my production code

## Focus on the following, using FA²STR
You want to shift to using FA²STR to think through your solutions methodically now. Here's how to use it on this assignment:

### UPFRONT

#### FIND
Responsibilities: 

List all the Responsibilities (from the description) in your text editor in a markdown file, on a notepad, or just ensure you know what they are (in your mind)

Examples: For each Responsibility, list some examples (or at the very least ensure you can visualize them)

#### ARCHITECTURE

Systems Thinking (Mental Model):
Simply remember that the System Under Test here is a mere input-output system, with data going in, and some data coming out immediately as a result.

Program By Wishful Thinking (upfront): 
* Think about the API you're about to build. 
* What's the shape of the object that goes in? 
* What's the shape of the object that comes out? 
* Can you visualize it in your mind? 
* If it's too complex, use a piece of paper, comments, or a whiteboarding tool to draw it out.

#### AUTOMATE
We've already got this set up. No need to do anything else. Your TypeScript + Jest config is all you need.

### EMERGENT

#### SPECIFY
Specify exactly ONE concrete example at a time

ex: it('should ....")

ex: test ("that it knows how to ....")

#### TEST

##### Test code
Write the test that expresses the specification
* Use the Think Backwards technique, starting from the Assert and working back up to the Act and then the Arrange (Arrange-Act-Assert Backwards).
* Use the Program By Wishful Thinking (emergent) technique to design an API that is understandable and pleasing to use.

##### Production code 
Write the simplest, minimum amount of code to make your test pass
* Fake It
* Obvious Implementation

#### REFINE
When you encounter duplication three times
* **Refactor your test code:** 
  * Use Parameterization (it.each) and the Encapsulate What Varies design principle to express the Responsibility as your Specification and your Examples as the input data
* **Refactor your production code:** 
  * Refactor to the simplest possible abstraction you can think of which fixes your duplication problem.
