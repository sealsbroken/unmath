// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::{AddToFive, CountToThree};
fn prepare(s: &mut dyn Strategy) {
    for _ in 0..3 {
        if s.snapshot().phase != PhaseKind::Performing {
            break;
        }
        s.handle(Event::Table(TableObservation::stable(
            s.snapshot().target_count.unwrap(),
        )));
    }
}
fn count(s: &mut dyn Strategy, n: u32) {
    s.handle(Event::Numbers((1..=n).collect()));
    s.handle(Event::Heard(n));
}
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
fn old_destinations_are_ignored_and_completed_additions_survive_pause() {
    let mut config = SessionConfig::default();
    config
        .extra
        .insert("unmath.lesson2.mastered".into(), "2097151".into());
    config
        .extra
        .insert("unmath.lesson2.place".into(), "moon".into());
    let mut s = AddToFive::new(config);
    s.handle(Event::Start);
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    prepare(&mut s);
    count(&mut s, 2); // guided opening
    prepare(&mut s);
    count(&mut s, 3); // independent 1+2
    let summary = s
        .handle(Event::Stop)
        .into_iter()
        .find_map(|e| {
            if let Effect::SessionEnded(s) = e {
                Some(s)
            } else {
                None
            }
        })
        .unwrap();
    let mut config = SessionConfig::default();
    summary.apply_remembered(&mut config.extra);
    assert!(!config.extra.contains_key("unmath.lesson2.place"));
    assert!(!config.extra.contains_key("unmath.lesson2.mastered"));
    let mut resumed = AddToFive::new(config);
    resumed.handle(Event::Start);
    assert_ne!(resumed.snapshot().phase, PhaseKind::Meeting);
    assert_eq!(resumed.snapshot().level, 1);
    prepare(&mut resumed);
    assert_eq!(resumed.snapshot().expected_answer, Some(2)); // first remaining task is 1+1
}
#[test]
fn a_default_length_addition_lesson_finishes_with_a_guided_help_path() {
    let mut s = AddToFive::new(SessionConfig::default());
    s.handle(Event::Start);
    prepare(&mut s);
    count(&mut s, 2);
    prepare(&mut s);
    s.handle(Event::Said("help".into()));
    prepare(&mut s);
    count(&mut s, 3);
    for _ in 0..7 {
        prepare(&mut s);
        let n = s.snapshot().expected_answer.unwrap();
        count(&mut s, n);
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    assert_eq!(s.snapshot().tasks_completed, 9);
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
