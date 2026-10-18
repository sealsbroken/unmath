// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::{language::Language, strategy::Effect};

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
            | "ещё раз"
            | "еще раз"
            | "повтори"
            | "повтори ещё раз"
            | "повтори еще раз"
    )
}

/// Undo the whole attempted addition without asking a child to subtract a difference.
pub(crate) fn undo_added_toys(language: Language) -> String {
    match language {
        Language::English => "Too many toys were added. Put ALL the toys you just added back beside the table. Leave the toys that were here before.",
        Language::Russian => "Игрушек добавилось слишком много. Убери со стола ВСЕ новые игрушки. Те, что были здесь раньше, оставь.",
    }.into()
}

/// Attach the reassurance question once, immediately before the next teaching question.
pub(crate) fn preface_retry(pending: &mut bool, language: Language, question: Effect) -> Effect {
    match question {
        Effect::Say(text) if *pending => {
            *pending = false;
            let prefix = match language {
                Language::English => "Are you sure?",
                Language::Russian => "А ты уверен?",
            };
            Effect::Say(format!("{prefix} {text}"))
        }
        effect => effect,
    }
}

pub(crate) fn toy_amount(n: u32) -> String {
    format!("{n} {}", if n == 1 { "toy" } else { "toys" })
}
