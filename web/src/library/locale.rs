#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Locale {
    En,
}

impl Locale {
    pub fn parse_str(locale: &str) -> Self {
        match locale {
            "en" => Locale::En,
            _ => Locale::En,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Locale::En => "en",
        }
    }
}