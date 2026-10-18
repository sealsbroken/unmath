// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::{language::Language, strategy::*};
use unmath::curriculum::{Adventure, Lesson, NEXT_LESSONS};
fn config() -> SessionConfig {
    SessionConfig {
        max_tasks: 80,
        ..Default::default()
    }
}
fn start(s: &mut Adventure) {
    s.handle(Event::Start);
    s.handle(Event::Said("moon".into()));
}
fn count(s: &mut Adventure, n: u32) -> Vec<Effect> {
    if n > 0 {
        s.handle(Event::Numbers((1..=n).collect()));
    }
    s.handle(Event::Heard(n))
}
fn answer(s: &mut Adventure, lesson: Lesson) -> Vec<Effect> {
    let n = s.snapshot().expected_answer.unwrap();
    if lesson == Lesson::FireflyGates {
        count(s, n)
    } else {
        s.handle(Event::Heard(n))
    }
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
fn spoken(effects: &[Effect]) -> String {
    un_core::strategy::spoken(effects).join(" ")
}
#[test]
fn all_ten_lessons_register_once_and_next_eight_finish_in_default_session() {
    let mut registry = StrategyRegistry::linked();
    unmath::register(&mut registry);
    unmath::register(&mut registry);
    assert_eq!(
        registry
            .infos()
            .iter()
            .filter(|i| i.id.starts_with("unmath-"))
            .count(),
        10
    );
    for lesson in NEXT_LESSONS {
        let mut s = registry
            .create(lesson.id(), &SessionConfig::default())
            .unwrap();
        assert_eq!(
            s.handle(Event::Start)
                .iter()
                .filter(|e| matches!(e, Effect::Listen { .. }))
                .count(),
            1
        );
        s.handle(Event::Heard(2));
        let mut seen = std::collections::HashSet::new();
        for _ in 0..lesson.mission_count() {
            let snap = s.snapshot();
            assert_eq!(snap.phase, PhaseKind::Asking);
            assert!(snap.expected_answer.unwrap() <= 10);
            assert!(seen.insert(snap.task.unwrap()));
            let n = snap.expected_answer.unwrap();
            if lesson == Lesson::FireflyGates {
                s.handle(Event::Numbers((1..=n).collect()));
            }
            let effects = s.handle(Event::Heard(n));
            if s.snapshot().phase == PhaseKind::Finished {
                let end = summary(effects);
                assert!(!end.paused);
                assert_eq!(end.final_level as usize, lesson.mission_count());
                assert!(end
                    .records
                    .iter()
                    .all(|r| r.correct && r.task.contains("independent")));
                assert!(end.remember.values().all(String::is_empty));
            }
        }
        assert_eq!(s.snapshot().phase, PhaseKind::Finished, "{lesson:?}");
    }
}
#[test]
fn every_help_path_uses_real_toys_then_rechecks_the_same_mission() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        let original = s.snapshot().task.unwrap();
        s.handle(Event::Said("help".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        assert_eq!(s.snapshot().tasks_completed, 0);
        let mut steps = 0;
        while s.snapshot().target_count.is_some() {
            steps += 1;
            assert!(steps <= 3, "guide should terminate");
            let n = s.snapshot().target_count.unwrap();
            assert!(n <= lesson.required_toys());
            s.handle(Event::Table(TableObservation::stable(n)));
            assert_eq!(s.snapshot().phase, PhaseKind::Asking);
            count(&mut s, n);
        }
        assert_eq!(s.snapshot().level, 0, "guided work never earns mastery");
        assert_eq!(s.snapshot().phase, PhaseKind::Asking);
        assert_eq!(
            s.snapshot().task.unwrap().split(" (").next(),
            original.split(" (").next(),
            "keep the helped mission"
        );
        if lesson == Lesson::BridgeBuilders {
            answer(&mut s, lesson); // identify the unmatched row
            answer(&mut s, lesson); // link unmatched blocks to more/equal
            assert_eq!(s.snapshot().level, 0);
        }
        // The same mission must now pass independently.
        for _ in 0..lesson.mission_count() {
            answer(&mut s, lesson);
        }
        assert_eq!(s.snapshot().phase, PhaseKind::Finished, "{lesson:?}");
    }
}
#[test]
fn wrong_answers_skips_and_taps_cannot_fake_counting_or_guided_success() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        let n = s.snapshot().expected_answer.unwrap();
        if lesson == Lesson::FireflyGates {
            s.handle(Event::Heard(n));
        } else {
            s.handle(Event::Heard(99));
        }
        assert_eq!(s.snapshot().level, 0);
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        s.handle(Event::Said("help".into()));
        let target = s.snapshot().target_count.unwrap();
        s.handle(Event::Table(TableObservation::stable(target)));
        if target > 0 {
            s.handle(Event::Heard(target));
            assert_eq!(s.snapshot().correct, 0);
        }
        s.handle(Event::SkipRequested);
        assert_ne!(s.snapshot().phase, PhaseKind::Finished);
    }
}
#[test]
fn disturbed_table_discards_pending_sequence_without_scoring() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        s.handle(Event::Said("help".into()));
        let n = s.snapshot().target_count.unwrap();
        s.handle(Event::Table(TableObservation::stable(n)));
        s.handle(Event::Numbers((1..=n).collect()));
        s.handle(Event::Table(TableObservation::default()));
        s.handle(Event::Heard(n));
        assert_eq!(s.snapshot().tasks_completed, 0);
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    }
}
#[test]
fn checkpoint_is_namespaced_and_resumes_only_remaining_missions_and_destination() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        answer(&mut s, lesson);
        let next = s.snapshot().task.unwrap();
        let end = summary(s.handle(Event::Stop));
        assert!(end.paused);
        assert_eq!(end.final_level, 1);
        let mut cfg = config();
        end.apply_remembered(&mut cfg.extra);
        assert!(cfg
            .extra
            .keys()
            .all(|k| k.starts_with(&format!("unmath.lesson{}.", lesson.number()))));
        let mut resumed = Adventure::new(lesson, cfg);
        let effects = resumed.handle(Event::Start);
        assert!(spoken(&effects).contains("moon"));
        assert_eq!(resumed.snapshot().task.unwrap(), next);
        assert_eq!(resumed.snapshot().level, 1);
        assert_eq!(resumed.snapshot().tasks_completed, 0);
    }
}
#[test]
fn low_materials_and_task_limit_pause_without_claiming_completion() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(
            lesson,
            SessionConfig {
                total_objects: lesson.required_toys() - 1,
                ..config()
            },
        );
        let end = summary(s.handle(Event::Start));
        assert!(end.paused);
        assert!(end.records.is_empty());
        let mut s = Adventure::new(
            lesson,
            SessionConfig {
                max_tasks: 1,
                ..config()
            },
        );
        start(&mut s);
        let end = summary(answer(&mut s, lesson));
        assert!(end.paused);
        assert_eq!(end.final_level, 1);
    }
}
#[test]
fn russian_help_break_and_repeated_silence_are_supported_for_every_lesson() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(
            lesson,
            SessionConfig {
                language: Language::Russian,
                ..config()
            },
        );
        let intro = s.handle(Event::Start);
        assert!(spoken(&intro).contains("приключение"));
        let q = s.handle(Event::Said("лес".into()));
        assert!(spoken(&q).chars().any(|c| ('А'..='я').contains(&c)));
        s.handle(Event::Said("помоги".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        s.handle(Event::Said("я устал".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Paused);
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        let repeated = s.handle(Event::RepeatRequested);
        assert!(repeated.iter().any(|e| matches!(e, Effect::Listen { .. })));
        for _ in 0..3 {
            s.handle(Event::NothingHeard);
        }
        assert_eq!(s.snapshot().phase, PhaseKind::Paused);
        assert_eq!(s.snapshot().tasks_completed, 0);
    }
}
#[test]
fn comparison_accepts_meaningful_words_in_both_languages() {
    let mut s = Adventure::new(Lesson::BridgeBuilders, config());
    start(&mut s);
    s.handle(Event::Said("second".into()));
    assert_eq!(s.snapshot().level, 1); // 2 vs 4, more
    s.handle(Event::Said("вторая".into()));
    assert_eq!(s.snapshot().level, 2); // 5 vs 3, fewer
    s.handle(Event::Said("same".into()));
    assert_eq!(s.snapshot().level, 3); // 3 vs 3
}
#[test]
fn count_to_ten_requires_every_number_and_two_separate_counts_to_ten() {
    let mut s = Adventure::new(Lesson::FireflyGates, config());
    start(&mut s);
    s.handle(Event::Numbers(vec![1, 2, 3, 4, 6]));
    s.handle(Event::Heard(6));
    assert_eq!(s.snapshot().level, 0);
    let mut s = Adventure::new(Lesson::FireflyGates, config());
    start(&mut s);
    for n in 6..=10 {
        count(&mut s, n);
    }
    assert_ne!(s.snapshot().phase, PhaseKind::Finished);
    assert_eq!(s.snapshot().expected_answer, Some(10));
    count(&mut s, 10);
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
}

#[test]
fn curriculum_answers_match_the_planned_arithmetic_and_comparison_choices() {
    let cases: [(Lesson, &[u32]); 8] = [
        (Lesson::CaveTrain, &[2, 2, 4, 0, 2, 5]),
        (Lesson::BridgeBuilders, &[2, 2, 3, 1, 1, 3]),
        (Lesson::ParcelDetectives, &[2, 3, 1, 1, 2, 0]),
        (Lesson::FireflyGates, &[6, 7, 8, 9, 10, 10]),
        (Lesson::SupplyLaunch, &[6, 7, 8, 9, 10, 8]),
        (Lesson::HomewardTrail, &[5, 4, 6, 5, 5, 0]),
        (Lesson::TenSeatFerry, &[5, 3, 7, 1, 10, 0]),
        (Lesson::FairShareCafe, &[1, 2, 3, 4, 5, 3]),
    ];
    for (lesson, expected_answers) in cases {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        for &answer in expected_answers {
            assert_eq!(s.snapshot().expected_answer, Some(answer), "{lesson:?}");
            if lesson == Lesson::FireflyGates {
                s.handle(Event::Numbers((1..=answer).collect()));
            }
            s.handle(Event::Heard(answer));
        }
        assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    }
}

#[test]
fn mistakes_preserve_the_current_mission_and_guided_stage() {
    for lesson in NEXT_LESSONS {
        let mut s = Adventure::new(lesson, config());
        start(&mut s);
        answer(&mut s, lesson);
        let before = s.snapshot();
        s.handle(Event::Heard(99));
        let after = s.snapshot();
        assert_eq!(
            after.task.as_ref().unwrap().split(" (").next(),
            before.task.as_ref().unwrap().split(" (").next(),
            "{lesson:?}"
        );
        assert_eq!(after.phase, PhaseKind::Performing);
        assert_eq!(after.level, 1);
        assert_eq!(after.expected_answer, None);
        s.handle(Event::Said("help".into()));
        let n = s.snapshot().target_count.unwrap();
        s.handle(Event::Table(TableObservation::stable(n)));
        count(&mut s, n);
        // Most lessons have multiple physical stages. A mistake at stage two
        // must not ask the child to build the initial group again.
        if let Some(target) = s.snapshot().target_count {
            s.handle(Event::Table(TableObservation::stable(target)));
            let before = s.snapshot();
            s.handle(Event::Heard(99));
            assert_eq!(s.snapshot().target_count, before.target_count, "{lesson:?}");
            assert_eq!(s.snapshot().task, before.task, "{lesson:?}");
            assert_eq!(s.snapshot().phase, PhaseKind::Asking);
            assert_eq!(s.snapshot().level, 1);
        }
    }
}
