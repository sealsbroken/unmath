// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::{language::Language, strategy::*};
use unmath::AddToFive;
fn lesson(language: Language) -> AddToFive {
    AddToFive::new(SessionConfig {
        language,
        max_tasks: 80,
        ..Default::default()
    })
}
fn table(s: &mut AddToFive, n: u32) -> Vec<Effect> {
    s.handle(Event::Table(TableObservation::stable(n)))
}
fn count(s: &mut AddToFive, ns: &[u32]) -> Vec<Effect> {
    s.handle(Event::Numbers(ns.to_vec()));
    s.handle(Event::Heard(*ns.last().unwrap()))
}
fn start(s: &mut AddToFive) {
    s.handle(Event::Start);
    count(s, &[1, 2]);
}
fn ready(s: &mut AddToFive) {
    for _ in 0..3 {
        if s.snapshot().phase != PhaseKind::Performing {
            break;
        }
        table(s, s.snapshot().target_count.unwrap());
    }
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
fn all_eight_examples_require_full_counts_then_totals_in_order() {
    for language in [Language::English, Language::Russian] {
        let mut s = lesson(language);
        start(&mut s);
        for (index, (a, b)) in [
            (1, 1),
            (1, 2),
            (2, 2),
            (2, 3),
            (3, 1),
            (1, 4),
            (3, 0),
            (1, 1),
        ]
        .into_iter()
        .enumerate()
        {
            assert!(s.snapshot().task.unwrap().contains(&format!("{a} + {b}")));
            assert_eq!(s.snapshot().level, index as u32);
            ready(&mut s);
            let es = count(&mut s, &(1..=a + b).collect::<Vec<_>>());
            assert_eq!(listens(&es), vec![ListenMode::Voice]);
            assert_eq!(
                s.snapshot().level,
                index as u32,
                "a count alone cannot finish an example"
            );
            let es = s.handle(Event::Heard(a + b));
            if index == 7 {
                let end = summary(es);
                assert!(!end.paused);
                assert_eq!(end.final_level, 8);
                assert_eq!(end.correct, 16);
            }
        }
        assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    }
}
#[test]
fn speech_check_retries_once_then_continues_without_scoring_or_moving_toys() {
    for response in [Event::NothingHeard, Event::Heard(9)] {
        let mut s = lesson(Language::Russian);
        let es = s.handle(Event::Start);
        assert!(spoken(&es).join(" ").contains("один, два, приём"));
        table(&mut s, 5);
        s.handle(response.clone());
        assert_eq!(s.snapshot().phase, PhaseKind::Meeting);
        s.handle(response);
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        assert_eq!(s.snapshot().target_count, Some(1));
        assert_eq!(s.snapshot().tasks_completed, 0);
    }
}
#[test]
fn wrong_count_and_wrong_total_teach_then_recheck_the_same_group() {
    for total_error in [false, true] {
        let mut s = lesson(Language::Russian);
        start(&mut s);
        ready(&mut s);
        if total_error {
            count(&mut s, &[1, 2]);
        }
        let es = s.handle(Event::Heard(5));
        assert!(spoken(&es).join(" ").contains("А ты уверен?"));
        assert!(s.snapshot().task.unwrap().contains("guided"));
        assert_eq!(s.snapshot().target_count, Some(2));
        count(&mut s, &[1, 2]);
        assert_eq!(s.snapshot().level, 0, "modeled counts give no mastery");
        assert!(s.snapshot().task.unwrap().contains("independent recount"));
        count(&mut s, &[1, 2]);
        assert_eq!(s.snapshot().level, 1);
    }
}
#[test]
fn guided_failure_models_once_and_final_failure_pauses_with_progress() {
    let mut s = lesson(Language::Russian);
    start(&mut s);
    ready(&mut s);
    count(&mut s, &[1, 2]);
    s.handle(Event::Heard(2));
    ready(&mut s);
    count(&mut s, &[0, 1, 2, 3]);
    let es = count(&mut s, &[1, 3]);
    assert!(spoken(&es).join(" ").contains("Посмотри и послушай"));
    assert!(s.snapshot().task.unwrap().contains("independent recount"));
    let end = summary(count(&mut s, &[1, 3]));
    assert!(end.paused);
    assert_eq!(end.final_level, 1);
    let mut cfg = SessionConfig::default();
    end.apply_remembered(&mut cfg.extra);
    let mut resumed = AddToFive::new(cfg);
    start(&mut resumed);
    assert_eq!(resumed.snapshot().level, 1);
    assert!(resumed.snapshot().task.unwrap().contains("1 + 2"));
}
#[test]
fn setup_corrections_are_specific_and_do_not_score_or_repeat_every_frame() {
    let mut s = lesson(Language::Russian);
    start(&mut s);
    let es = table(&mut s, 0);
    assert!(spoken(&es).join(" ").contains("мало"));
    let es = table(&mut s, 4);
    assert!(spoken(&es).join(" ").contains("слишком много"));
    assert!(table(&mut s, 4).is_empty());
    assert_eq!(s.snapshot().tasks_completed, 0);
    table(&mut s, 1);
    table(&mut s, 2);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
}
#[test]
fn interrupted_total_resumes_total_without_reusing_the_old_count() {
    let mut s = lesson(Language::English);
    start(&mut s);
    ready(&mut s);
    count(&mut s, &[1, 2]);
    s.handle(Event::Numbers(vec![2]));
    s.handle(Event::Table(TableObservation::default()));
    table(&mut s, 2);
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().tasks_completed, 1);
    assert_eq!(s.snapshot().level, 0);
    assert_eq!(
        listens(&s.handle(Event::RepeatRequested)),
        vec![ListenMode::Voice]
    );
    s.handle(Event::Heard(2));
    assert_eq!(s.snapshot().level, 1);
}
#[test]
fn counts_allow_complete_repetitions_but_reject_zero_skips_and_totals_only() {
    for ns in [
        vec![2],
        vec![0, 1, 2],
        vec![1, 3],
        vec![1, 2, 2],
        vec![1, 2, 1],
    ] {
        let mut s = lesson(Language::English);
        start(&mut s);
        ready(&mut s);
        count(&mut s, &ns);
        assert_eq!(s.snapshot().correct, 0);
        assert!(s.snapshot().task.unwrap().contains("guided"));
    }
    let mut s = lesson(Language::English);
    start(&mut s);
    ready(&mut s);
    count(&mut s, &[1, 2, 1, 2]);
    assert_eq!(s.snapshot().correct, 1);
    assert_eq!(s.snapshot().level, 0);
    count(&mut s, &[2, 2]);
    assert_eq!(s.snapshot().level, 1);
}
#[test]
fn timeouts_follow_bounded_corrections_and_task_budget_is_respected() {
    let mut s = lesson(Language::English);
    start(&mut s);
    ready(&mut s);
    for _ in 0..3 {
        s.handle(Event::NothingHeard);
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Paused);
    let mut s = AddToFive::new(SessionConfig {
        max_tasks: 1,
        ..Default::default()
    });
    start(&mut s);
    ready(&mut s);
    let end = summary(count(&mut s, &[1, 2]));
    assert!(end.paused);
    assert_eq!(end.tasks_completed, 1);
    assert_eq!(end.final_level, 0);
}
#[test]
fn zero_addition_needs_no_camera_change_and_completion_uses_configured_name() {
    let mut cfg = SessionConfig {
        child_name: Some("Вероника".into()),
        language: Language::Russian,
        max_tasks: 40,
        ..Default::default()
    };
    cfg.extra
        .insert("unmath.lesson2.graph.v4.index".into(), "6".into());
    let mut s = AddToFive::new(cfg);
    start(&mut s);
    let es = table(&mut s, 3);
    assert!(spoken(&es).join(" ").contains("Добавь ноль"));
    count(&mut s, &[1, 2, 3]);
    s.handle(Event::Heard(3));
    ready(&mut s);
    count(&mut s, &[1, 2]);
    let es = s.handle(Event::Heard(2));
    assert!(spoken(&es).join(" ").contains("Вероника"));
    assert!(!summary(es).paused);
}
