// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::{AddToFive, ADDITION_ID};
fn lesson() -> AddToFive {
    AddToFive::new(SessionConfig {
        max_tasks: 80,
        ..Default::default()
    })
}
fn table(s: &mut AddToFive, n: u32) -> Vec<Effect> {
    s.handle(Event::Table(TableObservation::stable(n)))
}
fn count(s: &mut AddToFive, ns: &[u32]) -> Vec<Effect> {
    assert!(s.handle(Event::Numbers(ns.to_vec())).is_empty());
    s.handle(Event::Heard(*ns.last().unwrap()))
}
fn demo(s: &mut AddToFive) {
    s.handle(Event::Start);
    assert_eq!(s.snapshot().target_count, Some(1));
    table(s, 1);
    assert_eq!(s.snapshot().target_count, Some(2));
    table(s, 2);
    count(s, &[1, 2]);
    assert_eq!(s.snapshot().level, 0);
    assert_eq!(s.snapshot().target_count, None);
}
fn summary(effects: Vec<Effect>) -> SessionSummary {
    effects
        .into_iter()
        .find_map(|e| {
            if let Effect::SessionEnded(s) = e {
                Some(s)
            } else {
                None
            }
        })
        .unwrap()
}
#[test]
fn all_twenty_one_facts_are_required_and_bounded_by_five() {
    let mut s = lesson();
    demo(&mut s);
    let mut tasks = std::collections::HashSet::new();
    for level in 0..21 {
        let snap = s.snapshot();
        assert_eq!(snap.level, level);
        assert!(snap.expected_answer.unwrap() <= 5);
        assert!(tasks.insert(snap.task.unwrap()));
        let effects = s.handle(Event::Heard(snap.expected_answer.unwrap()));
        if level == 20 {
            let end = summary(effects);
            assert!(!end.paused);
            assert_eq!(end.final_level, 21);
            assert_eq!(
                end.records
                    .iter()
                    .filter(|r| r.task.contains("independent"))
                    .count(),
                21
            );
        }
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    assert!(s.handle(Event::Heard(5)).is_empty());
}
#[test]
fn wrong_answer_branches_to_toys_and_guided_success_does_not_master_fact() {
    let mut s = lesson();
    demo(&mut s);
    // First check is 0 + 1. A wrong answer starts with an empty table.
    s.handle(Event::Heard(4));
    assert_eq!(s.snapshot().target_count, Some(0));
    table(&mut s, 0);
    assert_eq!(s.snapshot().target_count, Some(1));
    table(&mut s, 1);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    count(&mut s, &[1]);
    assert_eq!(s.snapshot().level, 0);
    assert!(s.snapshot().task.unwrap().contains("1 + 0"));
}
#[test]
fn adding_zero_needs_no_table_change_and_empty_sum_is_answerable() {
    let mut s = lesson();
    demo(&mut s);
    s.handle(Event::Heard(1)); // 0+1 -> 1+0
    s.handle(Event::Heard(2)); // wrong -> guided 1+0
    let effects = table(&mut s, 1);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    assert!(effects
        .iter()
        .any(|e| matches!(e, Effect::Say(t) if t.contains("Add zero"))));
    count(&mut s, &[1]);
    // Resume with all facts except 0+0 already mastered.
    let mut config = SessionConfig::default();
    config.extra.insert(
        "unmath.lesson2.mastered".into(),
        (((1u32 << 21) - 1) & !1).to_string(),
    );
    let mut zero = AddToFive::new(config);
    zero.handle(Event::Start);
    zero.handle(Event::Heard(1));
    let effects = table(&mut zero, 0);
    assert!(effects.iter().any(|e| matches!(
        e,
        Effect::Listen {
            mode: ListenMode::Voice,
            ..
        }
    )));
    zero.handle(Event::Heard(0)); // guided only
    assert_eq!(zero.snapshot().level, 20);
    assert!(!summary(zero.handle(Event::Heard(0))).paused);
}
#[test]
fn count_event_pair_does_not_answer_following_check() {
    let mut s = lesson();
    demo(&mut s);
    assert_eq!(s.snapshot().tasks_completed, 1);
    assert_eq!(s.snapshot().level, 0);
    s.handle(Event::Numbers(vec![1]));
    s.handle(Event::Heard(1));
    assert_eq!(s.snapshot().tasks_completed, 2);
    assert_eq!(s.snapshot().level, 1);
}
#[test]
fn table_change_cancels_guided_answer_and_taps_cannot_replace_guided_count() {
    let mut s = lesson();
    s.handle(Event::Start);
    table(&mut s, 1);
    table(&mut s, 2);
    s.handle(Event::Numbers(vec![1, 2]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().tasks_completed, 0);
    table(&mut s, 1);
    table(&mut s, 2);
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().tasks_completed, 1);
    assert_eq!(s.snapshot().correct, 0);
    assert_eq!(s.snapshot().level, 0);
}
#[test]
fn limit_pauses_and_checkpoint_resumes_only_unmastered_facts() {
    let mut s = lesson();
    demo(&mut s);
    s.handle(Event::Heard(1));
    let end = summary(s.handle(Event::Stop));
    assert!(end.paused);
    let mut config = SessionConfig {
        max_tasks: 1,
        ..Default::default()
    };
    end.apply_remembered(&mut config.extra);
    let mut resumed = AddToFive::new(config);
    resumed.handle(Event::Start);
    assert_eq!(resumed.snapshot().level, 1);
    assert!(!resumed.snapshot().task.unwrap().contains("0 + 1"));
    let expected = resumed.snapshot().expected_answer.unwrap();
    let end = summary(resumed.handle(Event::Heard(expected)));
    assert!(end.paused);
    assert_eq!(end.final_level, 2);
}
#[test]
fn silence_repeat_skip_and_russian_registration() {
    let mut registry = StrategyRegistry::linked();
    unmath::register(&mut registry);
    assert!(registry
        .create(ADDITION_ID, &SessionConfig::default())
        .is_some());
    let mut s = AddToFive::new(SessionConfig {
        language: un_core::language::Language::Russian,
        ..Default::default()
    });
    let effects = s.handle(Event::Start);
    assert!(effects
        .iter()
        .any(|e| matches!(e, Effect::Say(t) if t.contains("игрушками"))));
    assert_eq!(effects, s.handle(Event::RepeatRequested));
    assert!(summary(s.handle(Event::SkipRequested)).paused);
    let mut s = lesson();
    demo(&mut s);
    for _ in 0..3 {
        s.handle(Event::NothingHeard);
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Paused);
    assert_eq!(s.snapshot().tasks_completed, 1);
}
