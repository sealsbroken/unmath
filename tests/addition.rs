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
fn ready(s: &mut AddToFive) -> Vec<Effect> {
    let mut effects = vec![];
    for _ in 0..3 {
        if s.snapshot().phase != PhaseKind::Performing {
            break;
        }
        let n = s.snapshot().target_count.unwrap();
        effects.extend(table(s, n));
    }
    effects
}
fn count(s: &mut AddToFive, ns: &[u32]) -> Vec<Effect> {
    assert!(s.handle(Event::Numbers(ns.to_vec())).is_empty());
    s.handle(Event::Heard(*ns.last().unwrap()))
}
fn demo(s: &mut AddToFive) {
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    assert_eq!(s.snapshot().target_count, Some(1));
    table(s, 1);
    assert_eq!(s.snapshot().target_count, Some(2));
    table(s, 2);
    count(s, &[1, 2]);
    assert_eq!(s.snapshot().level, 0);
    ready(s);
}
fn summary(es: Vec<Effect>) -> SessionSummary {
    es.into_iter()
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
fn seven_baskets_require_one_number_per_toy_without_a_tutor_model() {
    let mut s = lesson();
    demo(&mut s);
    for (level, n) in [3, 4, 5, 4, 5, 3, 2].into_iter().enumerate() {
        let effects = ready(&mut s);
        assert!(!effects
            .iter()
            .any(|e| matches!(e, Effect::HighlightCount(_))));
        assert_eq!(s.snapshot().expected_answer, Some(n));
        assert_eq!(s.snapshot().level, level as u32);
        let effects = count(&mut s, &(1..=n).collect::<Vec<_>>());
        if level == 6 {
            let end = summary(effects);
            assert!(!end.paused);
            assert_eq!(end.final_level, 7);
            assert_eq!(end.correct, 8);
        }
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
}
#[test]
fn final_number_taps_and_broken_counts_do_not_complete_a_picnic_mission() {
    for ns in [vec![3], vec![1, 3], vec![1, 2, 2, 3], vec![1, 2, 3, 4]] {
        let mut s = lesson();
        demo(&mut s);
        count(&mut s, &ns);
        assert_eq!(s.snapshot().level, 0);
        assert_eq!(s.snapshot().target_count, Some(1));
    }
    let mut s = lesson();
    demo(&mut s);
    s.handle(Event::Heard(3));
    assert_eq!(s.snapshot().level, 0);
}
#[test]
fn full_count_is_collected_before_the_next_basket_can_be_answered() {
    let mut s = lesson();
    demo(&mut s);
    let effects = s.handle(Event::RepeatRequested);
    assert!(effects.iter().any(|e| matches!(
        e,
        Effect::Listen {
            mode: ListenMode::Counting,
            ..
        }
    )));
    assert!(un_core::strategy::spoken(&effects)
        .join(" ")
        .contains("one by one"));
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().tasks_completed, 2);
    assert_eq!(s.snapshot().level, 1);
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    s.handle(Event::Heard(4));
    assert_eq!(s.snapshot().tasks_completed, 2);
}
#[test]
fn guided_success_returns_to_another_basket_but_earns_no_mastery() {
    let mut s = lesson();
    demo(&mut s);
    s.handle(Event::Heard(4));
    assert_eq!(s.snapshot().target_count, Some(1));
    ready(&mut s);
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().level, 0);
    assert!(s.snapshot().task.unwrap().contains("2 + 2"));
}
#[test]
fn adding_zero_needs_no_table_change_but_still_requires_counting_each_toy() {
    let mut cfg = SessionConfig::default();
    cfg.extra.insert(
        "unmath.lesson2.one_by_one.v3".into(),
        ((1u32 << 6) - 1).to_string(),
    );
    let mut s = AddToFive::new(cfg);
    s.handle(Event::Start);
    let effects = ready(&mut s);
    assert!(un_core::strategy::spoken(&effects)
        .join(" ")
        .contains("empty basket"));
    assert_eq!(s.snapshot().expected_answer, Some(3));
    assert!(!summary(count(&mut s, &[1, 2, 3])).paused);
}
#[test]
fn table_changes_cancel_an_independent_count_without_scoring_it() {
    let mut s = lesson();
    demo(&mut s);
    s.handle(Event::Numbers(vec![1, 2, 3]));
    s.handle(Event::Table(TableObservation::default()));
    s.handle(Event::Heard(3));
    assert_eq!(s.snapshot().tasks_completed, 1);
    assert_eq!(s.snapshot().level, 0);
    ready(&mut s);
    count(&mut s, &[1, 2, 3]);
    assert_eq!(s.snapshot().level, 1);
}
#[test]
fn resume_saves_only_completed_one_by_one_baskets() {
    let mut s = lesson();
    demo(&mut s);
    count(&mut s, &[1, 2, 3]);
    let end = summary(s.handle(Event::Stop));
    assert!(end.paused);
    let mut cfg = SessionConfig {
        max_tasks: 1,
        ..Default::default()
    };
    end.apply_remembered(&mut cfg.extra);
    let mut resumed = AddToFive::new(cfg);
    resumed.handle(Event::Start);
    ready(&mut resumed);
    assert_eq!(resumed.snapshot().level, 1);
    let n = resumed.snapshot().expected_answer.unwrap();
    let end = summary(count(&mut resumed, &(1..=n).collect::<Vec<_>>()));
    assert!(end.paused);
    assert_eq!(end.final_level, 2);
}
#[test]
fn russian_prompts_registration_and_silence_work() {
    let mut registry = StrategyRegistry::linked();
    unmath::register(&mut registry);
    assert!(registry
        .create(ADDITION_ID, &SessionConfig::default())
        .is_some());
    let mut s = AddToFive::new(SessionConfig {
        language: un_core::language::Language::Russian,
        ..Default::default()
    });
    demo(&mut s);
    let effects = s.handle(Event::RepeatRequested);
    assert!(un_core::strategy::spoken(&effects)
        .join(" ")
        .contains("по очереди и считай"));
    for _ in 0..3 {
        s.handle(Event::NothingHeard);
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Paused);
}
