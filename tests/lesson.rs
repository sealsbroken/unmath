// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::{CountToFive, ID};
fn lesson() -> CountToFive {
    CountToFive::new(SessionConfig {
        max_tasks: 40,
        ..Default::default()
    })
}
fn start(s: &mut CountToFive) -> Vec<Effect> {
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    s.handle(Event::Table(TableObservation::stable(3)))
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
    start(&mut s);
    assert_eq!(s.snapshot().target_count, Some(3));
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().target_count, Some(4));
    table(&mut s, 4);
    count(&mut s, &[1, 2, 3, 4]);
    assert_eq!(s.snapshot().target_count, Some(5));
    table(&mut s, 5);
    count(&mut s, &[1, 2, 3, 4, 5]);
    count(&mut s, &[1, 2, 3, 4, 5]);
    assert_eq!(s.snapshot().tasks_completed, 4);
    s.handle(Event::Heard(5));
    let effects = count(&mut s, &[1, 2, 3, 4, 5]);
    assert!(effects.iter().any(
        |e| matches!(e,Effect::SessionEnded(summary) if !summary.paused && summary.correct==6)
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
        start(&mut s);
        count(&mut s, &ns);
        assert_eq!(s.snapshot().correct, 0);
        assert_eq!(s.snapshot().target_count, Some(1));
    }
    let mut s = lesson();
    start(&mut s);
    s.handle(Event::Heard(5));
    assert_eq!(s.snapshot().correct, 0);
}
#[test]
fn practice_cycles_back_to_an_independent_check() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1, 3]);
    for n in 1..=5 {
        table(&mut s, n);
        count(&mut s, &(1..=n).collect::<Vec<_>>());
    }
    assert_eq!(s.snapshot().target_count, Some(5));
    assert!(s.snapshot().task.unwrap().contains("First"));
    assert_ne!(s.snapshot().phase, PhaseKind::Finished);
}
#[test]
fn unstable_table_invalidates_pending_count() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1, 2, 3]);
    table(&mut s, 4);
    s.handle(Event::Numbers(vec![1, 2, 3, 4]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(4));
    assert_eq!(s.snapshot().tasks_completed, 1);
    table(&mut s, 4);
    s.handle(Event::Heard(4));
    assert_eq!(s.snapshot().correct, 1);
}

#[test]
fn task_limit_and_silence_pause_without_mastery() {
    let mut s = CountToFive::new(SessionConfig {
        max_tasks: 1,
        ..Default::default()
    });
    start(&mut s);
    let result = count(&mut s, &[1, 2, 3]);
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
    assert_eq!(resumed.snapshot().target_count, Some(4));
    let mut s = lesson();
    start(&mut s);
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
    start(&mut s);
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
    let first = start(&mut s);
    assert!(first
        .iter()
        .any(|e| matches!(e, Effect::Say(text) if text.contains("по одному"))));
    assert_eq!(first, s.handle(Event::RepeatRequested));
    s.handle(Event::SkipRequested);
    assert_eq!(s.snapshot().correct, 0);
    assert_eq!(s.snapshot().target_count, Some(1));
}

#[test]
fn the_first_count_waits_for_real_toys_and_invalidates_when_the_table_changes() {
    let mut s = lesson();
    s.handle(Event::Start);
    s.handle(Event::Heard(1));
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    count(&mut s, &[1, 2, 3, 4, 5]);
    assert_eq!(s.snapshot().tasks_completed, 0);
    table(&mut s, 3);
    let effects = s.handle(Event::RepeatRequested);
    assert!(un_core::strategy::spoken(&effects)
        .join(" ")
        .contains("one number for each"));
    s.handle(Event::Numbers(vec![1, 2, 3]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(3));
    assert_eq!(s.snapshot().tasks_completed, 0);
}
#[test]
fn pointing_at_the_toys_does_not_restart_the_question() {
    let mut s = lesson();
    start(&mut s);
    let asked = s.snapshot();
    assert_eq!(asked.phase, PhaseKind::Asking);
    // A hand over the toys: the settled count holds but the table is not still.
    let moving = TableObservation {
        count: Some(3),
        is_stable: false,
        groups: 1,
    };
    for _ in 0..10 {
        assert!(s.handle(Event::Table(moving)).is_empty());
        assert!(s
            .handle(Event::Table(TableObservation::stable(3)))
            .is_empty());
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().correct, 1);
}

#[test]
fn three_available_toys_start_the_game_and_save_the_next_stop_for_later() {
    let mut s = CountToFive::new(SessionConfig {
        total_objects: 3,
        ..Default::default()
    });
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    table(&mut s, 3);
    let effects = count(&mut s, &[1, 2, 3]);
    let summary = effects
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
    assert_eq!(summary.correct, 1);
    let mut config = SessionConfig::default();
    summary.apply_remembered(&mut config.extra);
    let mut resumed = CountToFive::new(config);
    resumed.handle(Event::Start);
    // No five-toy mastery was earned by the three-toy warm-up.
    assert_eq!(resumed.snapshot().level, 0);
    resumed.handle(Event::Said("forest".into()));
    assert_eq!(resumed.snapshot().target_count, Some(4));
}

#[test]
fn the_three_and_four_toy_warmups_cannot_complete_a_count_to_five_lesson() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1, 2, 3]);
    table(&mut s, 4);
    count(&mut s, &[1, 2, 3, 4]);
    assert_ne!(s.snapshot().phase, PhaseKind::Finished);
    assert_eq!(s.snapshot().target_count, Some(5));
    assert_eq!(s.snapshot().level, 0);
}
#[test]
fn toys_already_on_the_table_move_the_warmup_on() {
    let opening = |s: &mut CountToFive| {
        s.handle(Event::Start);
        s.handle(Event::Said("forest".into()))
    };
    // Four toys out before the game: the first count is of four, with the opening line.
    let mut s = lesson();
    table(&mut s, 4);
    let said = un_core::strategy::spoken(&opening(&mut s)).join(" ");
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    assert_eq!(s.snapshot().target_count, Some(4));
    assert!(said.contains("first four friends"), "{said}");
    // Five out: straight to the count to five.
    let mut s = lesson();
    table(&mut s, 5);
    opening(&mut s);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    assert_eq!(s.snapshot().target_count, Some(5));
    // Seven out: five are needed, the tutor says how many to take away.
    let mut s = lesson();
    table(&mut s, 7);
    let said = un_core::strategy::spoken(&opening(&mut s)).join(" ");
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    assert!(said.contains("Take 2 away"), "{said}");
    table(&mut s, 5);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    // Two out: one more is needed for the three-toy start.
    let mut s = lesson();
    table(&mut s, 2);
    let said = un_core::strategy::spoken(&opening(&mut s)).join(" ");
    assert!(said.contains("Add 1 more"), "{said}");
    // A fourth toy put out while waiting: the game starts at four.
    table(&mut s, 4);
    assert_eq!(s.snapshot().target_count, Some(4));
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
}
