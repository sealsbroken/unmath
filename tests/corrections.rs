// SPDX-License-Identifier: LGPL-3.0-or-later
//! A mistake must open an interactive teaching path, not replay a question stem.
use un_core::{language::Language, strategy::*};
use unmath::curriculum::{Adventure, Lesson};
fn text(es: &[Effect]) -> String {
    spoken(es).join(" ")
}
fn ready(s: &mut dyn Strategy) {
    for _ in 0..5 {
        if s.snapshot().phase != PhaseKind::Performing {
            return;
        }
        s.handle(Event::Table(TableObservation::stable(
            s.snapshot().target_count.unwrap(),
        )));
    }
    assert_ne!(s.snapshot().phase, PhaseKind::Performing);
}
fn count(s: &mut dyn Strategy, n: u32) {
    s.handle(Event::Numbers((1..=n).collect()));
    s.handle(Event::Heard(n));
}
#[test]
fn all_ten_lessons_teach_after_a_mistake_before_a_fresh_followup() {
    let mut registry = StrategyRegistry::default();
    unmath::register(&mut registry);
    for info in registry.infos() {
        for language in [Language::English, Language::Russian] {
            let cfg = SessionConfig {
                language,
                max_tasks: 100,
                ..Default::default()
            };
            let mut s = registry.create(&info.id, &cfg).unwrap();
            s.handle(Event::Start);
            s.handle(Event::Said("forest".into()));
            ready(s.as_mut());
            if info.id == unmath::ADDITION_ID {
                count(s.as_mut(), 2); // opening demo is not an independent mission
                ready(s.as_mut());
            }
            let before = s.snapshot();
            let original = s.handle(Event::RepeatRequested);
            let original_prompt = spoken(&original).last().unwrap().to_string();
            let correction = s.handle(Event::Heard(99));
            assert!(
                !spoken(&correction)
                    .iter()
                    .any(|line| *line == original_prompt),
                "{} replays the original stem",
                info.id
            );
            assert_eq!(s.snapshot().level, before.level);
            let mut steps = 0;
            loop {
                ready(s.as_mut());
                let snap = s.snapshot();
                if steps > 0
                    && !snap.task.as_ref().unwrap().contains("guided")
                    && !snap.task.as_ref().unwrap().contains("Repair")
                {
                    break;
                }
                assert!(steps < 7, "{}: {:?}", info.id, snap);
                if snap.target_count.is_some() {
                    count(s.as_mut(), snap.expected_answer.unwrap());
                } else {
                    s.handle(Event::Heard(snap.expected_answer.unwrap()));
                }
                steps += 1;
                assert_eq!(
                    s.snapshot().level,
                    before.level,
                    "{}: guided correction earns no mastery",
                    info.id
                );
            }
            let followup = s.handle(Event::RepeatRequested);
            assert!(
                !spoken(&followup)
                    .iter()
                    .any(|line| *line == original_prompt),
                "{} follow-up repeats original question",
                info.id
            );
            assert!(s.snapshot().tasks_completed > before.tasks_completed + 1);
        }
    }
}

#[test]
fn comparison_separates_counting_pairing_vocabulary_and_fewer_choice_in_russian() {
    let cfg = SessionConfig {
        language: Language::Russian,
        max_tasks: 100,
        ..Default::default()
    };
    let mut s = Adventure::new(Lesson::BridgeBuilders, cfg);
    s.handle(Event::Start);
    s.handle(Event::Said("лес".into()));
    s.handle(Event::Said("вторая".into())); // 2 versus 4, MORE: completed
    let original = text(&s.handle(Event::RepeatRequested)); // 5 versus 3, FEWER
    let correction = s.handle(Event::Said("первая".into()));
    assert!(!text(&correction).contains(&original));
    assert_eq!(s.snapshot().target_count, Some(5));
    ready(&mut s);
    count(&mut s, 5);
    assert_eq!(s.snapshot().target_count, Some(3));
    ready(&mut s);
    count(&mut s, 3);
    let pairing = text(&s.handle(Event::RepeatRequested));
    assert!(pairing.contains("БЕЗ пары"));
    s.handle(Event::Said("первая".into())); // more blocks, but not the answer to FEWER
    assert!(text(&s.handle(Event::RepeatRequested)).contains("БОЛЬШЕ или МЕНЬШЕ"));
    let correction = s.handle(Event::Said("меньше".into())); // vocabulary misconception
    assert!(text(&correction).contains("Больше — значит"));
    assert_eq!(s.snapshot().level, 1);
    s.handle(Event::Said("больше".into()));
    let followup = text(&s.handle(Event::RepeatRequested));
    assert!(followup.contains("Нам нужно МЕНЬШЕ"));
    s.handle(Event::Said("первая".into())); // still confuses goal: retain counted groups
    assert!(s.snapshot().task.unwrap().contains("comparison words"));
    assert_eq!(s.snapshot().target_count, None); // don't recount the already learned groups
    s.handle(Event::Said("больше".into()));
    s.handle(Event::Said("вторая".into()));
    assert_eq!(s.snapshot().level, 2);
}

#[test]
fn arithmetic_misconceptions_get_different_concrete_corrections() {
    let cases = [
        (Lesson::CaveTrain, 3, "before anyone stopped"),
        (Lesson::CaveTrain, 1, "travelers who stopped"),
        (Lesson::ParcelDetectives, 3, "whole group"),
        (Lesson::ParcelDetectives, 1, "were here before"),
        (Lesson::SupplyLaunch, 5, "just one ship"),
        (Lesson::FairShareCafe, 2, "all the treats"),
    ];
    for (lesson, wrong, expected_hint) in cases {
        let mut s = Adventure::new(
            lesson,
            SessionConfig {
                max_tasks: 100,
                ..Default::default()
            },
        );
        s.handle(Event::Start);
        s.handle(Event::Said("forest".into()));
        let effects = s.handle(Event::Heard(wrong));
        assert!(
            text(&effects).contains(expected_hint),
            "{lesson:?}: {}",
            text(&effects)
        );
        assert_eq!(s.snapshot().phase, PhaseKind::Performing);
        assert_eq!(s.snapshot().level, 0);
    }
}

#[test]
fn equal_groups_are_matched_without_teaching_subtraction() {
    let mut s = Adventure::new(
        Lesson::BridgeBuilders,
        SessionConfig {
            max_tasks: 100,
            ..Default::default()
        },
    );
    s.handle(Event::Start);
    s.handle(Event::Said("forest".into()));
    s.handle(Event::Said("second".into()));
    s.handle(Event::Said("second".into())); // now compare 3 and 3
    let effects = s.handle(Event::Said("first".into()));
    assert!(text(&effects).contains("equal groups"));
    ready(&mut s);
    count(&mut s, 3);
    ready(&mut s);
    count(&mut s, 3);
    s.handle(Event::Said("same".into())); // no unmatched row
    assert!(text(&s.handle(Event::RepeatRequested)).contains("Every block has a partner"));
    s.handle(Event::Said("same".into())); // equal amounts
    assert_eq!(s.snapshot().level, 2); // teaching still earns no mastery
    s.handle(Event::Said("same".into())); // fresh team-choice follow-up
    assert_eq!(s.snapshot().level, 3);
}
