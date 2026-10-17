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
    pub fn from_words(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "forest" | "the forest" | "лес" | "в лес" => Some(Self::Forest),
            "moon" | "the moon" | "луна" | "на луну" => Some(Self::Moon),
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
                "You choose our adventure! Forest or moon? Say forest or moon, or tap one or two."
                    .into()
            }
            Language::Russian => {
                "Выбирай приключение! Лес или Луна? Скажи лес или луна, или нажми один или два."
                    .into()
            }
        }),
        Effect::listen(ListenMode::Voice, config.answer_timeout),
    ]
}
pub(crate) fn wants_break(words: &str) -> bool {
    matches!(
        words.trim().to_lowercase().as_str(),
        "stop" | "break" | "i'm tired" | "стоп" | "перерыв" | "я устал" | "я устала"
    )
}

pub(crate) fn wants_help(words: &str) -> bool {
    matches!(
        words.trim().to_lowercase().as_str(),
        "help" | "help me" | "i don't know" | "помоги" | "не знаю"
    )
}
