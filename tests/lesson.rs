// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::{CountToThree, ID};
fn lesson() -> CountToThree {
    CountToThree::new(SessionConfig {
        max_tasks: 40,
        ..Default::default()
    })
}
fn start(s: &mut CountToThree) -> Vec<Effect> {
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    s.handle(Event::Table(TableObservation::stable(1)))
}
fn count(s: &mut CountToThree, ns: &[u32]) -> Vec<Effect> {
    assert!(s.handle(Event::Numbers(ns.to_vec())).is_empty());
    s.handle(Event::Heard(*ns.last().unwrap()))
}
fn table(s: &mut CountToThree, n: u32) {
    s.handle(Event::Table(TableObservation::stable(n)));
}
#[test]
fn two_independent_counts_separated_by_toy_activity_pass() {
    let mut s = lesson();
    start(&mut s);
    assert_eq!(s.snapshot().target_count, Some(1));
    count(&mut s, &[1]);
    assert_eq!(s.snapshot().target_count, Some(2));
    table(&mut s, 2);
    count(&mut s, &[1, 2]);
    assert_eq!(s.snapshot().target_count, Some(3));
    table(&mut s, 3);
    count(&mut s, &[1, 2, 3]);
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().tasks_completed, 4);
    s.handle(Event::Heard(3));
    let effects = count(&mut s, &[1, 2, 3]);
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
    s.handle(Event::Heard(3));
    assert_eq!(s.snapshot().correct, 0);
}
#[test]
fn a_mistake_does_not_reset_the_counting_progression() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1]);
    table(&mut s, 2);
    let before = s.snapshot();
    count(&mut s, &[1, 3]);
    assert_eq!(s.snapshot().target_count, Some(2));
    assert_ne!(s.snapshot().task, before.task);
    assert!(s.snapshot().task.unwrap().contains("Repair"));
    assert_eq!(s.snapshot().correct, 1);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    count(&mut s, &[1, 2]);
    assert_eq!(
        s.snapshot().target_count,
        Some(2),
        "repair is a teaching step"
    );
    count(&mut s, &[1, 2]);
    assert_eq!(s.snapshot().target_count, Some(3));
    table(&mut s, 3);
    count(&mut s, &[1, 2, 3]);
    let before = s.snapshot();
    count(&mut s, &[0, 1, 2, 3]);
    assert_ne!(s.snapshot().task, before.task);
    assert!(s.snapshot().task.unwrap().contains("Repair"));
    assert_eq!(s.snapshot().target_count, Some(3));
    assert_eq!(s.snapshot().correct, 4);
    count(&mut s, &[1, 2, 3]);
    count(&mut s, &[1, 2, 3]);
    assert!(s.snapshot().task.unwrap().contains("Total"));
}
#[test]
fn unstable_table_invalidates_pending_count() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1]);
    table(&mut s, 2);
    s.handle(Event::Numbers(vec![1, 2]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().tasks_completed, 1);
    table(&mut s, 2);
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().correct, 1);
}

#[test]
fn task_limit_and_silence_pause_without_mastery() {
    let mut s = CountToThree::new(SessionConfig {
        max_tasks: 1,
        ..Default::default()
    });
    start(&mut s);
    let result = count(&mut s, &[1]);
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
    let mut resumed = CountToThree::new(config);
    resumed.handle(Event::Start);
    assert_eq!(resumed.snapshot().target_count, Some(2));
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
    let effects = s.handle(Event::Said("help".into()));
    assert!(effects.contains(&Effect::HighlightCount(1)));
    assert!(effects
        .iter()
        .any(|e| matches!(e, Effect::Say(text) if text.contains("Now try by yourself"))));
}

#[test]
fn russian_prompts_and_repeat_preserve_question() {
    let mut s = CountToThree::new(SessionConfig {
        language: un_core::language::Language::Russian,
        ..Default::default()
    });
    let first = start(&mut s);
    assert!(first
        .iter()
        .any(|e| matches!(e, Effect::Say(text) if text.contains("Один"))));
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
    count(&mut s, &[1]);
    assert_eq!(s.snapshot().tasks_completed, 0);
    table(&mut s, 1);
    let effects = s.handle(Event::RepeatRequested);
    assert!(un_core::strategy::spoken(&effects)
        .join(" ")
        .contains("Say one"));
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
        count: Some(1),
        is_stable: false,
        groups: 1,
    };
    for _ in 0..10 {
        assert!(s.handle(Event::Table(moving)).is_empty());
        assert!(s
            .handle(Event::Table(TableObservation::stable(1)))
            .is_empty());
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    count(&mut s, &[1]);
    assert_eq!(s.snapshot().correct, 1);
}

#[test]
fn one_available_toy_start_the_game_and_save_the_next_stop_for_later() {
    let mut s = CountToThree::new(SessionConfig {
        total_objects: 1,
        ..Default::default()
    });
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    table(&mut s, 1);
    let effects = count(&mut s, &[1]);
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
    let mut resumed = CountToThree::new(config);
    resumed.handle(Event::Start);
    // The one-toy warm-up did not earn three-toy mastery.
    assert_eq!(resumed.snapshot().level, 0);
    resumed.handle(Event::Said("forest".into()));
    assert_eq!(resumed.snapshot().target_count, Some(2));
}

#[test]
fn the_one_and_two_toy_warmups_cannot_complete_a_count_to_three_lesson() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1]);
    table(&mut s, 2);
    count(&mut s, &[1, 2]);
    assert_ne!(s.snapshot().phase, PhaseKind::Finished);
    assert_eq!(s.snapshot().target_count, Some(3));
    assert_eq!(s.snapshot().level, 0);
}
#[test]
fn extra_toys_never_skip_the_one_toy_opening() {
    for n in [2, 3, 4, 5, 7] {
        let mut s = lesson();
        table(&mut s, n);
        s.handle(Event::Start);
        let effects = s.handle(Event::Said("forest".into()));
        assert_eq!(s.snapshot().target_count, Some(1));
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        assert!(un_core::strategy::spoken(&effects)
            .join(" ")
            .contains("other toys aside"));
        table(&mut s, 1);
        assert_eq!(s.snapshot().phase, PhaseKind::Asking);
        assert_eq!(s.snapshot().tasks_completed, 0);
    }
}

#[test]
fn old_five_toy_checkpoints_do_not_skip_the_new_opening() {
    let mut config = SessionConfig::default();
    config
        .extra
        .insert("unmath.lesson1.one_by_one.v2".into(), "true".into());
    config
        .extra
        .insert("unmath.lesson1.warmup.v3".into(), "5".into());
    let mut s = CountToThree::new(config);
    start(&mut s);
    assert_eq!(s.snapshot().target_count, Some(1));
    assert_eq!(s.snapshot().correct, 0);
}

#[test]
fn help_returns_to_the_question_where_the_child_asked() {
    let mut s = lesson();
    start(&mut s);
    count(&mut s, &[1]);
    table(&mut s, 2);
    count(&mut s, &[1, 2]);
    table(&mut s, 3);
    count(&mut s, &[1, 2, 3]);
    let original = s.snapshot().task;
    s.handle(Event::Said("help".into()));
    for n in 1..=3 {
        table(&mut s, n);
        count(&mut s, &(1..=n).collect::<Vec<_>>());
    }
    assert_eq!(s.snapshot().task, original);
    assert_eq!(s.snapshot().target_count, Some(3));
}
