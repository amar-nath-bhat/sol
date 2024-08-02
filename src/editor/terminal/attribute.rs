use crossterm::style::Color;

use super::super::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb { r: 0, g: 0, b: 0 }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 251,
                    b: 0,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb { r: 0, g: 0, b: 0 }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 165,
                    b: 0,
                }),
            },
            AnnotationType::Number => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 99,
                    b: 71,
                }),
                background: None,
            },
            AnnotationType::Keyword => Self {
                foreground: Some(Color::Rgb {
                    r: 137,
                    g: 206,
                    b: 255,
                }),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(Color::Rgb { r: 0, g: 140, b: 0 }),
                background: None,
            },
            AnnotationType::KnownValue => Self {
                foreground: Some(Color::Rgb {
                    r: 100,
                    g: 0,
                    b: 140,
                }),
                background: None,
            },
            AnnotationType::Char => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 191,
                    b: 0,
                }),
                background: None,
            },
            AnnotationType::LifetimeSpecifier => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 191,
                    b: 0,
                }),
                background: None,
            },
            AnnotationType::Comment => Self {
                foreground: Some(Color::Rgb { r: 0, g: 100, b: 0 }),
                background: None,
            },
            AnnotationType::MultilineComment => Self {
                foreground: Some(Color::Rgb {
                    r: 0,
                    g: 100,
                    b: 100,
                }),
                background: None,
            },
            AnnotationType::String => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 0,
                    b: 255,
                }),
                background: None,
            },
        }
    }
}
