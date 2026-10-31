//! The answer type returned by puzzle solvers.

use std::fmt;

/// A puzzle answer: most are numbers, some are text such as a hash or a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Num(u64),
    Text(String),
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::Num(n)
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}

impl fmt::Display for Answer {
    /// Delegates to the inner value, so width and alignment flags apply.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => n.fmt(f),
            Self::Text(s) => s.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_number() {
        assert_eq!(Answer::from(42u64), Answer::Num(42));
        let answer: Answer = 7.into();
        assert_eq!(answer, Answer::Num(7));
    }

    #[test]
    fn test_from_text() {
        assert_eq!(Answer::from("abc"), Answer::Text("abc".to_owned()));
        assert_eq!(
            Answer::from(String::from("x,y")),
            Answer::Text("x,y".to_owned())
        );
    }

    #[test]
    fn test_number_and_text_differ() {
        assert_ne!(Answer::from(1u64), Answer::from("1"));
    }

    #[test]
    fn test_display() {
        assert_eq!(Answer::from(123u64).to_string(), "123");
        assert_eq!(Answer::from("a1b2").to_string(), "a1b2");
    }

    #[test]
    fn test_display_honours_width() {
        assert_eq!(format!("{:<5}|", Answer::from(42u64)), "42   |");
        assert_eq!(format!("{:<5}|", Answer::from("ab")), "ab   |");
    }
}
