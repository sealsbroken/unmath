// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::{
    language::Language,
    strategy::{Effect, ListenMode, SessionConfig},
};

/// A real child choice, remembered across a paused adventure.
#[derive(Clone, Copy, Default)]
pub(crate) enum Place {
    #[default]
    Forest,
    Moon,
}
impl Place {
    pub fn from_number(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::Forest),
            2 => Some(Self::Moon),
            _ => None,
        }
    }
    /// The place the child named, wherever in what they said ("лес лес", "давай в лес!", "the
    /// moon please"); nothing when they named neither, or both.
    pub fn from_words(s: &str) -> Option<Self> {
        let words = normalized(s);
        let forest = words
            .split(' ')
            .any(|w| matches!(w, "forest" | "лес" | "леса" | "лесу" | "лесной"));
        let moon = words
            .split(' ')
            .any(|w| matches!(w, "moon" | "луна" | "луну" | "луне" | "луны"));
        match (forest, moon) {
            (true, false) => Some(Self::Forest),
            (false, true) => Some(Self::Moon),
            _ => None,
        }
    }
    pub fn saved(self) -> &'static str {
        match self {
            Self::Forest => "forest",
            Self::Moon => "moon",
        }
    }
    pub fn vehicle(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Forest, Language::English) => "bus",
            (Self::Moon, Language::English) => "rocket",
            (Self::Forest, Language::Russian) => "автобус",
            (Self::Moon, Language::Russian) => "ракету",
        }
    }
    pub fn friends(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Forest, Language::English) => "bears",
            (Self::Moon, Language::English) => "aliens",
            (Self::Forest, Language::Russian) => "мишек",
            (Self::Moon, Language::Russian) => "инопланетян",
        }
    }
    pub fn cheer(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Forest, Language::English) => "Beep beep!",
            (Self::Moon, Language::English) => "Whoosh!",
            (Self::Forest, Language::Russian) => "Би-би!",
            (Self::Moon, Language::Russian) => "Вжух!",
        }
    }
}
pub(crate) fn invitation(config: &SessionConfig) -> Vec<Effect> {
    vec![
        Effect::Say(match config.language {
            Language::English => {
                "You choose our adventure! Forest or moon? Say forest or moon.".into()
            }
            Language::Russian => {
                "Выбирай приключение! В лес или на Луну? Скажи: лес или Луна.".into()
            }
        }),
        Effect::listen(ListenMode::Voice, config.answer_timeout),
    ]
}
pub(crate) fn wants_break(words: &str) -> bool {
    matches!(
        normalized(words).as_str(),
        "stop"
            | "please stop"
            | "stop please"
            | "break"
            | "i'm tired"
            | "i am tired"
            | "i want a break"
            | "стоп"
            | "перерыв"
            | "я устал"
            | "я устала"
    )
}

pub(crate) fn wants_help(words: &str) -> bool {
    matches!(
        normalized(words).as_str(),
        "help"
            | "help me"
            | "please help"
            | "help me please"
            | "i don't know"
            | "i dont know"
            | "помоги"
            | "помоги пожалуйста"
            | "не знаю"
    )
}

/// Recognizers commonly add punctuation, curly apostrophes, or extra spaces.
pub(crate) fn normalized(words: &str) -> String {
    words
        .replace('’', "'")
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '\'' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
pub(crate) fn wants_restart(words: &str) -> bool {
    matches!(
        normalized(words).as_str(),
        "start again"
            | "restart"
            | "let me start again"
            | "let's start again"
            | "сначала"
            | "начать сначала"
            | "давай сначала"
    )
}
