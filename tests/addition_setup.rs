// SPDX-License-Identifier: LGPL-3.0-or-later
//! Physical addition errors are undone as a whole action, never by subtracting an excess.
use un_core::{language::Language, strategy::*};
use unmath::{
    curriculum::{Adventure, Lesson},
    AddToFive,
};

fn table(s: &mut dyn Strategy, n: u32) -> Vec<Effect> {
    s.handle(Event::Table(TableObservation::stable(n)))
}
fn count(s: &mut dyn Strategy, n: u32) {
    s.handle(Event::Numbers((1..=n).collect()));
    s.handle(Event::Heard(n));
}
fn words(effects: &[Effect]) -> String {
    spoken(effects).join(" ")
}
fn check_reset(s: &mut dyn Strategy, base: u32, target: u32, language: Language) {
    let before = s.snapshot();
    // A moving observation is not an overshoot.
    let moving = s.handle(Event::Table(TableObservation {
        count: Some(target + 1),
        is_stable: false,
        groups: 1,
    }));
    assert!(!words(&moving).contains("Too many toys"));
    assert!(!words(&moving).contains("слишком много"));
    let effects = table(s, target + 1);
    let speech = words(&effects);
    assert!(
        speech.contains(if language == Language::English {
            "ALL the toys you just added"
        } else {
            "ВСЕ новые игрушки"
        }),
        "{speech}"
    );
    assert!(!effects.iter().any(|e| matches!(
        e,
        Effect::Listen { .. } | Effect::SessionEnded(_) | Effect::Celebrate
    )));
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    assert_eq!(s.snapshot().target_count, Some(base));
    assert_eq!(s.snapshot().level, before.level);
    assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
    assert!(table(s, target + 1).is_empty()); // no warning every camera frame
    assert!(
        words(&s.handle(Event::RepeatRequested)).contains(if language == Language::English {
            "ALL the toys you just added"
        } else {
            "ВСЕ новые игрушки"
        })
    );
    // Removing only the excess must NOT bypass undoing the attempted addition.
    if base != target {
        table(s, target);
    }
    count(s, target);
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
    if base > 0 {
        table(s, base - 1); // original toys were also removed: still wait for the starting group
        assert_eq!(s.snapshot().target_count, Some(base));
    }
    let retry = table(s, base);
    assert!(!retry.iter().any(|e| matches!(e, Effect::Listen { .. })) || base == target);
    assert_eq!(s.snapshot().target_count, Some(target));
    table(s, target);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
}

#[test]
fn adding_three_instead_of_two_requires_undo_then_a_fresh_addition() {
    for language in [Language::English, Language::Russian] {
        let mut cfg = SessionConfig {
            language,
            max_tasks: 40,
            ..Default::default()
        };
        cfg.extra
            .insert("unmath.lesson2.one_by_one.v3".into(), "1".into());
        let mut s = AddToFive::new(cfg);
        s.handle(Event::Start);
        table(&mut s, 1);
        assert_eq!(s.snapshot().target_count, Some(3)); // one original; add two
        check_reset(&mut s, 1, 3, language);
        count(&mut s, 3);
        assert_eq!(s.snapshot().level, 2);
    }
}

#[test]
fn guided_additions_and_missing_quantities_undo_without_losing_the_guided_stage() {
    for language in [Language::English, Language::Russian] {
        for lesson in [
            Lesson::SupplyLaunch,
            Lesson::ParcelDetectives,
            Lesson::TenSeatFerry,
        ] {
            let cfg = SessionConfig {
                language,
                max_tasks: 40,
                ..Default::default()
            };
            let mut s = Adventure::new(lesson, cfg);
            s.handle(Event::Start);

            s.handle(Event::Said("help".into()));
            let base = s.snapshot().target_count.unwrap();
            table(&mut s, base);
            count(&mut s, base);
            let target = s.snapshot().target_count.unwrap();
            assert!(target > base);
            let stage = s.snapshot().task;
            check_reset(&mut s, base, target, language);
            assert_eq!(s.snapshot().task, stage);
            count(&mut s, target);
            assert_eq!(s.snapshot().level, 0); // guided correction never earns mastery
        }
    }
}

#[test]
fn zero_addition_overshoot_can_be_undone_and_paused_without_inventing_a_score() {
    let mut cfg = SessionConfig::default();
    cfg.extra
        .insert("unmath.lesson2.one_by_one.v3".into(), "63".into());
    let mut s = AddToFive::new(cfg);
    s.handle(Event::Start);
    table(&mut s, 3); // plus zero; already listening
    let effects = table(&mut s, 4);
    assert!(words(&effects).contains("ALL the toys"));
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    assert_eq!(s.snapshot().target_count, Some(3));
    let effects = s.handle(Event::Said("break".into()));
    assert!(effects.iter().any(|e| matches!(e, Effect::SessionEnded(summary) if summary.paused && summary.final_level == 6 && summary.tasks_completed == 0)));
}

#[test]
fn empty_starting_groups_and_zero_additions_restore_the_exact_baseline() {
    for (lesson, index, base, target) in [
        (Lesson::ParcelDetectives, 4, 0, 2),
        (Lesson::TenSeatFerry, 4, 0, 10),
        (Lesson::ParcelDetectives, 5, 5, 5),
        (Lesson::TenSeatFerry, 5, 10, 10),
    ] {
        let mut cfg = SessionConfig {
            max_tasks: 40,
            ..Default::default()
        };
        cfg.extra.insert(
            format!("unmath.lesson{}.adventure.v1.index", lesson.number()),
            index.to_string(),
        );
        let mut s = Adventure::new(lesson, cfg);
        s.handle(Event::Start);

        s.handle(Event::Said("help".into()));
        table(&mut s, base);
        count(&mut s, base);
        // In zero-addition cases the second group is already correct and listening.
        assert_eq!(s.snapshot().target_count, Some(target));
        check_reset(&mut s, base, target, Language::English);
        assert_eq!(s.snapshot().level, 0);
    }
}

#[test]
fn overshoot_discards_a_pending_count_and_can_be_corrected_repeatedly() {
    let mut cfg = SessionConfig {
        max_tasks: 40,
        ..Default::default()
    };
    cfg.extra
        .insert("unmath.lesson2.one_by_one.v3".into(), "1".into());
    let mut s = AddToFive::new(cfg);
    s.handle(Event::Start);
    table(&mut s, 1);
    table(&mut s, 3);
    s.handle(Event::Numbers(vec![1, 2, 3]));
    s.handle(Event::Table(TableObservation::default())); // hand obscures the extra toy
    table(&mut s, 4);
    table(&mut s, 1);
    // A second overfull attempt needs its own undo; no progress is lost or invented.
    table(&mut s, 4);
    assert_eq!(s.snapshot().target_count, Some(1));
    table(&mut s, 1);
    table(&mut s, 3);
    s.handle(Event::Heard(3)); // trailing answer from the canceled count
    assert_eq!(s.snapshot().tasks_completed, 0);
    count(&mut s, 3);
    assert_eq!(s.snapshot().level, 2);
}
