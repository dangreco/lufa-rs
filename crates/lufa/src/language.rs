use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    English,
    French,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::English => write!(f, "en"),
            Self::French => write!(f, "fr"),
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "en" => Language::English,
            "fr" => Language::French,
            _ => panic!("invalid language"),
        }
    }
}

impl From<Language> for &str {
    fn from(value: Language) -> Self {
        match value {
            Language::English => "en",
            Language::French => "fr",
        }
    }
}
