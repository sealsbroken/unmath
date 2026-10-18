// SPDX-License-Identifier: LGPL-3.0-or-later
//! Grade complete toy counts, including repetitions of the same complete answer.
use un_core::{language::Language, strategy::Effect};

pub(crate) fn valid_count(numbers: &[u32], expected: u32, heard: Option<u32>) -> bool {
    expected > 0
        && heard == Some(expected)
        && !numbers.is_empty()
        && numbers.len().is_multiple_of(expected as usize)
        && numbers
            .iter()
            .enumerate()
            .all(|(i, n)| *n == (i % expected as usize) as u32 + 1)
}
pub(crate) fn bounded_count(mut numbers: Vec<u32>) -> Vec<u32> {
    // Every lesson counts at most ten. Keep ample diagnostic context, not an
    // unbounded transcript in every persisted task record. Mark truncation invalid.
    if numbers.len() > 32 {
        // A child may repeat a complete count many times. Validate the WHOLE
        // response before compacting it, so late mistakes cannot disappear.
        let cycle = numbers
            .iter()
            .skip(1)
            .position(|n| *n == 1)
            .map(|i| i + 1)
            .unwrap_or(numbers.len());
        if cycle <= 10 && valid_count(&numbers, cycle as u32, numbers.last().copied()) {
            numbers.truncate(cycle);
            return numbers;
        }
        numbers.truncate(32);
        numbers.push(u32::MAX);
    }
    numbers
}
pub(crate) fn count_feedback(numbers: &[u32], expected: u32, language: Language) -> Effect {
    let (en, ru) = if numbers.first() == Some(&0) || numbers.contains(&0) {
        ("Zero means no toys. Point to the first toy and say one. Let's try again.",
         "Ноль означает, что игрушек нет. Покажи на первую игрушку и скажи: один или раз. Попробуем ещё раз.")
    } else if numbers.is_empty() || numbers.len() == 1 && expected > 1 {
        ("I need to hear each number as you point to each toy. Start with one on the first toy.",
         "Нужно услышать каждое число, пока ты показываешь на игрушки. Покажи на первую игрушку и скажи: один или раз.")
    } else if numbers.first() != Some(&1) {
        (
            "For these toys, start with one on the first toy. Let's count them again.",
            "Покажи на первую игрушку и скажи: один или раз. Давай посчитаем ещё раз.",
        )
    } else if numbers.windows(2).any(|w| w[0] == w[1]) {
        (
            "We repeated a number. Give each toy just one number. Let's start again.",
            "Мы повторили число. Каждой игрушке — одно число. Начнём сначала.",
        )
    } else if numbers.len() > expected as usize {
        ("Let's point to each toy once and stop after the last toy. Try again from one.",
         "Покажем на каждую игрушку один раз и остановимся после последней. Начни снова с одного.")
    } else {
        (
            "Let's count again in order, one number for each toy. Take your time.",
            "Посчитаем снова по порядку, по одному числу на игрушку. Не спеши.",
        )
    };
    Effect::Say(match language {
        Language::English => en.into(),
        Language::Russian => ru.into(),
    })
}

/// Saved bit masks must not turn an invalid value (for example u32::MAX) into
/// all missions completed just by masking its low bits.
pub(crate) fn saved_mask(value: Option<&String>, all: u32) -> u32 {
    value
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|n| *n < all && n & !all == 0)
        .unwrap_or(0)
}
pub(crate) fn safe_config(
    mut config: un_core::strategy::SessionConfig,
) -> un_core::strategy::SessionConfig {
    // Engine listens convert these to Duration; NaN/infinity/negative values panic.
    if !config.answer_timeout.is_finite()
        || config.answer_timeout <= 0.0
        || config.answer_timeout > 3600.0
    {
        config.answer_timeout = un_core::strategy::SessionConfig::default().answer_timeout;
    }
    config
}

/// Completed speech counts arrive as Numbers followed by Heard. If a repeat,
/// restart, or table change cancels a cached Numbers, discard that known tail
/// rather than scoring it against the replacement question.
#[derive(Default)]
pub(crate) struct CountBuffer {
    numbers: Option<Vec<u32>>,
    discarded_tail: Option<u32>,
}
impl CountBuffer {
    pub fn set(&mut self, numbers: Vec<u32>) {
        self.numbers = Some(bounded_count(numbers));
        self.discarded_tail = None;
    }
    pub fn take(&mut self) -> Option<Vec<u32>> {
        self.discarded_tail = None;
        self.numbers.take()
    }
    pub fn reset(&mut self) {
        if let Some(numbers) = self.numbers.take() {
            self.discarded_tail = numbers.last().copied();
        }
    }
    pub fn ignore_discarded_tail(&mut self, n: u32) -> bool {
        self.discarded_tail.take() == Some(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_answer_repetitions_are_valid_and_bounded_for_every_toy_count() {
        for n in 1..=10 {
            let answer: Vec<_> = (1..=n).collect();
            for repetitions in [1, 2, 3, 40, 1000] {
                let response = answer.repeat(repetitions);
                assert!(valid_count(&response, n, Some(n)));
                let stored = bounded_count(response);
                assert!(stored.len() <= 32);
                assert!(valid_count(&stored, n, Some(n)));
                assert!(!valid_count(&stored, n, Some(n + 1)));
            }
        }
    }

    #[test]
    fn repeats_do_not_hide_zero_skips_stutters_overcounts_or_incomplete_repetitions() {
        for response in [
            vec![],
            vec![2],
            vec![1, 1, 2],
            vec![1, 2, 2],
            vec![1, 2, 1],
            vec![1, 2, 1, 3],
            vec![1, 2, 3, 1, 2],
            vec![0, 1, 2, 1, 2],
            vec![1, 2, 0, 1, 2],
        ] {
            assert!(
                !valid_count(&response, 2, response.last().copied()),
                "{response:?}"
            );
        }
        let mut late_error = [1, 2].repeat(100);
        late_error[151] = 3;
        assert!(!valid_count(&bounded_count(late_error), 2, Some(2)));
        assert!(!valid_count(&[], 0, Some(0))); // zero uses the separate empty-set rule
    }
}
