use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum RegistrationError {
    EmptyName,
    InvalidAge { age: u8, minimum: u8 },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "name cannot be empty"),
            Self::InvalidAge { age, minimum } => {
                write!(formatter, "age {age} is below the minimum of {minimum}")
            }
        }
    }
}

impl Error for RegistrationError {}

fn register_user(name: &str, age: u8) -> Result<User, RegistrationError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(RegistrationError::EmptyName);
    }

    const MINIMUM_AGE: u8 = 13;
    if age < MINIMUM_AGE {
        return Err(RegistrationError::InvalidAge {
            age,
            minimum: MINIMUM_AGE,
        });
    }

    Ok(User {
        name: name.to_string(),
        age,
    })
}

fn main() {
    for (name, age) in [("Ferris", 16), ("", 20), ("Rustacean", 10)] {
        match register_user(name, age) {
            Ok(user) => println!("Registered {} (age {})", user.name, user.age),
            Err(error) => println!("Could not register {name:?}: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_valid_user() {
        assert_eq!(
            register_user(" Ferris ", 16),
            Ok(User {
                name: String::from("Ferris"),
                age: 16,
            })
        );
    }

    #[test]
    fn returns_a_specific_age_error() {
        assert_eq!(
            register_user("Ferris", 10),
            Err(RegistrationError::InvalidAge {
                age: 10,
                minimum: 13,
            })
        );
    }
}
