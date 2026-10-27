// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::CountToThree;
#[test]
fn counting_starts_with_one_real_toy_and_keeps_spoken_help_and_break() {
    let mut s = CountToThree::new(SessionConfig::default());
    s.handle(Event::Start);
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    assert_eq!(s.snapshot().target_count, Some(1));
    let first = s.handle(Event::Table(TableObservation::stable(1)));
    assert!(spoken(&first).join(" ").contains("one toy"));
    s.handle(Event::Said("help".into()));
    assert_eq!(s.snapshot().target_count, Some(1));
    assert_eq!(s.snapshot().correct, 0);
    s.handle(Event::Said("I'm tired".into()));
    assert_eq!(s.snapshot().phase, PhaseKind::Paused);
}
#[test]
fn russian_repeat_request_with_raz_replays_without_scoring_an_answer() {
    let mut s = CountToThree::new(SessionConfig {
        language: un_core::language::Language::Russian,
        ..Default::default()
    });
    s.handle(Event::Start);
    s.handle(Event::Table(TableObservation::stable(1)));
    for words in ["ещё раз", "еще раз", "повтори ещё раз", "повтори"]
    {
        let before = s.snapshot();
        let effects = s.handle(Event::Said(words.into()));
        assert!(effects.iter().any(|e| matches!(e, Effect::Listen { .. })));
        assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
        assert_eq!(s.snapshot().correct, before.correct);
    }
}
