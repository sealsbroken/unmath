use un_core::strategy::*;
use unmath::{CountToFive, ID};
fn lesson() -> CountToFive {
    CountToFive::new(SessionConfig {
        max_tasks: 40,
        ..Default::default()
    })
}
fn count(s: &mut CountToFive, ns: &[u32]) -> Vec<Effect> {
    assert!(s.handle(Event::Numbers(ns.to_vec())).is_empty());
    s.handle(Event::Heard(*ns.last().unwrap()))
}
fn table(s: &mut CountToFive, n: u32) {
    s.handle(Event::Table(TableObservation::stable(n)));
}
#[test]
fn two_independent_counts_separated_by_toy_activity_pass() {
    let mut s = lesson();
    s.handle(Event::Start);
    count(&mut s, &[1, 2, 3, 4, 5]);
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    table(&mut s, 5);
    count(&mut s, &[1, 2, 3, 4, 5]);
    // The preceding Heard(5) answered only the counting question, not the total.
    assert_eq!(s.snapshot().tasks_completed, 2);
    s.handle(Event::Heard(5));
    let effects = count(&mut s, &[1, 2, 3, 4, 5]);
    assert!(effects.iter().any(
        |e| matches!(e, Effect::SessionEnded(summary) if !summary.paused && summary.correct == 4)
    ));
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
}
#[test]
fn last_number_taps_and_malformed_sequences_do_not_pass() {
    for ns in [
        vec![5],
        vec![1, 2, 4, 5],
        vec![1, 2, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6],
    ] {
        let mut s = lesson();
        s.handle(Event::Start);
        count(&mut s, &ns);
        assert_eq!(s.snapshot().correct, 0);
        assert_eq!(s.snapshot().target_count, Some(1));
    }
    let mut s = lesson();
    s.handle(Event::Start);
    s.handle(Event::Heard(5));
    assert_eq!(s.snapshot().correct, 0);
}
#[test]
fn practice_cycles_back_to_an_independent_check() {
    let mut s = lesson();
    s.handle(Event::Start);
    count(&mut s, &[1, 3]);
    for n in 1..=5 {
        table(&mut s, n);
        count(&mut s, &(1..=n).collect::<Vec<_>>());
    }
    assert_eq!(s.snapshot().target_count, None);
    assert!(s.snapshot().task.unwrap().contains("First"));
    assert_ne!(s.snapshot().phase, PhaseKind::Finished);
}
#[test]
fn unstable_table_invalidates_pending_count() {
    let mut s = lesson();
    s.handle(Event::Start);
    count(&mut s, &[1, 2, 3, 4, 5]);
    table(&mut s, 5);
    s.handle(Event::Numbers(vec![1, 2, 3, 4, 5]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(5));
    assert_eq!(s.snapshot().tasks_completed, 1);
    table(&mut s, 5);
    s.handle(Event::Heard(5));
    assert_eq!(s.snapshot().correct, 1);
}
#[test]
fn task_limit_and_silence_pause_without_mastery() {
    let mut s = CountToFive::new(SessionConfig {
        max_tasks: 1,
        ..Default::default()
    });
    s.handle(Event::Start);
    let result = count(&mut s, &[1, 2, 3, 4, 5]);
    let summary = result
        .into_iter()
        .find_map(|e| {
            if let Effect::SessionEnded(s) = e {
                Some(s)
            } else {
                None
            }
        })
        .unwrap();
    assert!(summary.paused);
    let mut config = SessionConfig::default();
    summary.apply_remembered(&mut config.extra);
    let mut resumed = CountToFive::new(config);
    resumed.handle(Event::Start);
    assert_eq!(resumed.snapshot().target_count, Some(5));
    let mut s = lesson();
    s.handle(Event::Start);
    for _ in 0..3 {
        s.handle(Event::NothingHeard);
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Paused);
}
#[test]
fn registration_deduplicates_and_creates() {
    let mut registry = StrategyRegistry::linked();
    unmath::register(&mut registry);
    unmath::register(&mut registry);
    assert_eq!(registry.infos().iter().filter(|i| i.id == ID).count(), 1);
    assert!(registry.create(ID, &SessionConfig::default()).is_some());
}

#[test]
fn help_is_modeled_after_the_required_toys_arrive() {
    let mut s = lesson();
    s.handle(Event::Start);
    count(&mut s, &[1, 3]);
    let effects = s.handle(Event::Table(TableObservation::stable(1)));
    assert!(effects.contains(&Effect::HighlightCount(1)));
    assert!(effects
        .iter()
        .any(|e| matches!(e, Effect::Say(text) if text.contains("Now try by yourself"))));
}

#[test]
fn russian_prompts_and_repeat_preserve_question() {
    let mut s = CountToFive::new(SessionConfig {
        language: un_core::language::Language::Russian,
        ..Default::default()
    });
    let first = s.handle(Event::Start);
    assert!(first
        .iter()
        .any(|e| matches!(e, Effect::Say(text) if text.contains("пяти"))));
    assert_eq!(first, s.handle(Event::RepeatRequested));
    s.handle(Event::SkipRequested);
    assert_eq!(s.snapshot().correct, 0);
    assert_eq!(s.snapshot().target_count, Some(1));
}
