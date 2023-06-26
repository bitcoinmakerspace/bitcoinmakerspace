#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Locale {
    En,
    Es,
    Fr,
    Pt,
}

impl Locale {
    pub fn parse_str(locale: &str) -> Self {
        match locale {
            "en" => Locale::En,
            "es" => Locale::Es,
            "fr" => Locale::Fr,
            "pt" => Locale::Pt,
            _ => Locale::En,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Locale::En => "en",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::Pt => "pt",
        }
    }
}