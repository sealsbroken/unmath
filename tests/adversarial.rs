// SPDX-License-Identifier: LGPL-3.0-or-later
//! Counterexamples to accidental passes, stale responses, and dead-end transitions.
use un_core::{language::Language, strategy::*};
fn ids() -> Vec<String> {
    let mut r = StrategyRegistry::default();
    unmath::register(&mut r);
    r.infos().into_iter().map(|i| i.id).collect()
}
/// The counting lessons. Lesson two (the bus) asks for single numbers, takes any speech in its
/// warm-up and waits out a silent pause on the camera: tests/addition.rs holds its rules.
fn counting_ids() -> Vec<String> {
    ids()
        .into_iter()
        .filter(|id| id != unmath::ADDITION_ID)
        .collect()
}
fn make(id: &str, config: SessionConfig) -> Box<dyn Strategy> {
    let mut r = StrategyRegistry::default();
    unmath::register(&mut r);
    r.create(id, &config).unwrap()
}
fn config(language: Language) -> SessionConfig {
    SessionConfig {
        language,
        max_tasks: 100,
        ..Default::default()
    }
}
fn prepare(s: &mut dyn Strategy) {
    for _ in 0..4 {
        if s.snapshot().phase != PhaseKind::Performing {
            break;
        }
        let n = s.snapshot().target_count.unwrap();
        s.handle(Event::Table(TableObservation::stable(n)));
    }
}
fn start(s: &mut dyn Strategy) {
    s.handle(Event::Start);

    if s.snapshot().phase == PhaseKind::Meeting {
        s.handle(Event::Numbers(vec![1, 2]));
        s.handle(Event::Heard(2));
    }
    prepare(s);
}
fn counting(id: &str, language: Language) -> Box<dyn Strategy> {
    let mut s = make(id, config(language));
    start(s.as_mut());
    if id != unmath::ID && id != unmath::ADDITION_ID && id != "unmath-count-to-ten" {
        s.handle(Event::Said("help".into()));
        prepare(s.as_mut());
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
    assert!(s.snapshot().expected_answer.unwrap() > 0);
    s
}
fn words(es: &[Effect]) -> String {
    spoken(es).join(" ")
}
fn respond(s: &mut dyn Strategy, ns: Vec<u32>, tail: u32) -> Vec<Effect> {
    s.handle(Event::Numbers(ns));
    s.handle(Event::Heard(tail))
}
#[test]
fn zero_start_is_rejected_and_explained_in_both_languages_in_all_ten_lessons() {
    for id in counting_ids() {
        for language in [Language::English, Language::Russian] {
            let mut s = counting(&id, language);
            let n = s.snapshot().expected_answer.unwrap();
            let before = s.snapshot();
            let effects = respond(s.as_mut(), (0..=n).collect(), n);
            assert_eq!(s.snapshot().correct, before.correct, "{id}");
            assert_eq!(s.snapshot().level, before.level, "{id}");
            assert!(
                words(&effects).contains(if language == Language::English {
                    "Zero means no toys"
                } else {
                    "Ноль означает"
                }),
                "{id}: {}",
                words(&effects)
            );
        }
    }
}
#[test]
fn malformed_count_edits_are_rejected_but_complete_answer_repetitions_are_legal() {
    for id in counting_ids() {
        let template = counting(&id, Language::English);
        let n = template.snapshot().expected_answer.unwrap();
        let good: Vec<_> = (1..=n).collect();
        let mut mutations = vec![
            vec![],
            vec![n + 1],
            vec![u32::MAX],
            vec![0; n as usize + 1],
            (2..=n + 1).collect(),
        ];
        for i in 0..=good.len() {
            let mut ns = good.clone();
            ns.insert(i, 0);
            mutations.push(ns);
        }
        for i in 0..good.len() {
            let mut ns = good.clone();
            ns.remove(i);
            mutations.push(ns);
            let mut ns = good.clone();
            ns.insert(i, ns[i]);
            mutations.push(ns);
        }
        for i in 0..good.len().saturating_sub(1) {
            let mut ns = good.clone();
            ns.swap(i, i + 1);
            mutations.push(ns);
        }
        let mut ns = good.clone();
        ns.push(n + 1);
        mutations.push(ns);
        let mut ns = good.clone();
        ns.reverse();
        mutations.push(ns);
        mutations.push(vec![1; 10000]);
        for ns in mutations {
            if !ns.is_empty()
                && ns.len().is_multiple_of(good.len())
                && ns.chunks(good.len()).all(|part| part == good)
            {
                continue; // repeating the entire correct answer is legal, including one toy
            }
            let mut s = counting(&id, Language::English);
            let before = s.snapshot();
            let tail = ns.last().copied().unwrap_or(n);
            respond(s.as_mut(), ns.clone(), tail);
            assert_eq!(s.snapshot().correct, before.correct, "accepted {id} {ns:?}");
            assert_eq!(s.snapshot().level, before.level, "{id}");
        }
        let mut s = counting(&id, Language::English);
        let before = s.snapshot();
        respond(s.as_mut(), good, n + 1);
        assert_eq!(
            s.snapshot().correct,
            before.correct,
            "mismatched tail: {id}"
        );
    }
}
#[test]
fn repeated_complete_answers_score_once_across_all_lessons_and_languages() {
    for id in counting_ids() {
        for language in [Language::English, Language::Russian] {
            for times in [2, 3, 100] {
                let mut s = counting(&id, language);
                let before = s.snapshot();
                let n = before.expected_answer.unwrap();
                let answer: Vec<_> = (1..=n).collect();
                respond(s.as_mut(), answer.repeat(times), n);
                assert_eq!(
                    s.snapshot().tasks_completed,
                    before.tasks_completed + 1,
                    "{id}"
                );
                assert_eq!(s.snapshot().correct, before.correct + 1, "{id}");
            }
        }
    }
}
#[test]
fn partial_chunks_never_score_and_pause_split_complete_counts_score_once() {
    for id in counting_ids() {
        let mut s = counting(&id, Language::English);
        let n = s.snapshot().expected_answer.unwrap();
        let before = s.snapshot();
        for v in 1..=n {
            s.handle(Event::Counted(vec![v]));
            s.handle(Event::Sound(false));
            assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
        }
        s.handle(Event::Numbers((1..=n).collect()));
        assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
        s.handle(Event::Heard(n));
        assert_eq!(
            s.snapshot().tasks_completed,
            before.tasks_completed + 1,
            "{id}"
        );
        assert_eq!(s.snapshot().correct, before.correct + 1);
    }
}
#[test]
fn repeat_restart_and_table_changes_discard_the_known_old_heard_tail() {
    for id in counting_ids() {
        for cancel in [
            Event::RepeatRequested,
            Event::Said("Start again!".into()),
            Event::Table(TableObservation::default()),
        ] {
            let mut s = counting(&id, Language::English);
            let n = s.snapshot().expected_answer.unwrap();
            let before = s.snapshot();
            // An independent spoken firefly count does not depend on the table.
            if matches!(cancel, Event::Table(_)) && before.target_count.is_none() {
                continue;
            }
            s.handle(Event::Numbers((1..=n).collect()));
            s.handle(cancel.clone());
            s.handle(Event::Heard(n));
            assert_eq!(
                s.snapshot().tasks_completed,
                before.tasks_completed,
                "stale tail scored after {cancel:?} in {id}"
            );
            assert_eq!(s.snapshot().level, before.level);
        }
    }
}
#[test]
fn zero_is_a_real_answer_for_an_empty_group_not_a_blanket_forbidden_number() {
    let mut s = make("unmath-subtract-to-five", config(Language::English));
    start(s.as_mut());
    for n in [2, 2, 4] {
        s.handle(Event::Heard(n));
    }
    assert_eq!(s.snapshot().expected_answer, Some(0));
    s.handle(Event::Numbers(vec![0]));
    s.handle(Event::Heard(0));
    assert_eq!(s.snapshot().level, 4);
    // Physical support for 2-2 must also accept an empty table and a spoken zero.
    let mut s = make("unmath-subtract-to-five", config(Language::English));
    start(s.as_mut());
    for n in [2, 2, 4] {
        s.handle(Event::Heard(n));
    }
    s.handle(Event::Said("help".into()));
    prepare(s.as_mut());
    respond(s.as_mut(), vec![1, 2], 2);
    prepare(s.as_mut());
    assert_eq!(s.snapshot().target_count, Some(0));
    s.handle(Event::Heard(0));
    assert_eq!(s.snapshot().level, 3);
}
#[test]
fn recognizer_punctuation_case_whitespace_and_curly_apostrophes_do_not_trap_children() {
    for id in ids() {
        let mut s = make(&id, config(Language::English));
        s.handle(Event::Start);
        s.handle(Event::Said("  MOON!  ".into()));
        prepare(s.as_mut());
        s.handle(Event::Said("  Please   help. ".into()));
        assert!(
            matches!(
                s.snapshot().phase,
                PhaseKind::Performing | PhaseKind::Asking | PhaseKind::Meeting
            ),
            "{id}"
        );
        assert_eq!(s.snapshot().tasks_completed, 0);
        s.handle(Event::Said("I’m tired.".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Paused, "{id}");
        let mut s = make(&id, config(Language::Russian));
        s.handle(Event::Start);
        s.handle(Event::Said("ЛУНА!".into()));
        prepare(s.as_mut());
        s.handle(Event::Said("Помоги, пожалуйста!".into()));
        s.handle(Event::Said("Я устала!".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Paused);
    }
}
#[test]
fn malformed_checkpoint_masks_never_create_completion_or_skip_all_missions() {
    for id in ids() {
        for value in [
            "4294967295",
            "2147483648",
            "-1",
            "NaN",
            "99999999999999999999999",
            "",
        ] {
            let mut cfg = config(Language::English);
            cfg.extra
                .insert("unmath.lesson2.one_by_one.v3".into(), value.into());
            for lesson in 3..=10 {
                cfg.extra
                    .insert(format!("unmath.lesson{lesson}.adventure.v1"), value.into());
                cfg.extra.insert(
                    format!("unmath.lesson{lesson}.adventure.v1.index"),
                    value.into(),
                );
            }
            let mut s = make(&id, cfg);
            s.handle(Event::Start);
            assert_ne!(s.snapshot().phase, PhaseKind::Finished, "{id} {value}");
            assert_eq!(s.snapshot().level, 0, "{id} {value}");
        }
    }
}
#[test]
fn insufficient_materials_or_zero_task_allowance_pauses_without_a_score() {
    for id in ids() {
        for cfg in [
            SessionConfig {
                total_objects: 0,
                ..config(Language::English)
            },
            SessionConfig {
                max_tasks: 0,
                ..config(Language::English)
            },
        ] {
            let mut s = make(&id, cfg);
            let effects = s.handle(Event::Start);
            assert_eq!(s.snapshot().phase, PhaseKind::Paused, "{id}");
            assert_eq!(s.snapshot().tasks_completed, 0);
            assert!(!effects.iter().any(|e| matches!(e, Effect::Listen { .. })));
        }
    }
}
#[test]
fn invalid_timeouts_do_not_emit_panicking_duration_inputs() {
    for id in ids() {
        for timeout in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            -1.0,
            0.0,
            f64::MAX,
        ] {
            let mut s = make(
                &id,
                SessionConfig {
                    answer_timeout: timeout,
                    ..config(Language::English)
                },
            );
            let mut effects = s.handle(Event::Start);
            effects.extend(s.handle(Event::Heard(1)));
            prepare(s.as_mut());
            effects.extend(s.handle(Event::RepeatRequested));
            for e in effects {
                if let Effect::Listen { timeout, .. } = e {
                    assert!(
                        timeout.is_finite() && timeout > 0.0 && timeout <= 3600.0,
                        "{id}"
                    );
                }
            }
        }
    }
}
#[test]
fn obsolete_destination_words_do_not_create_a_choice_or_score() {
    for id in counting_ids() {
        let mut s = make(&id, config(Language::English));
        s.handle(Event::Start);
        let before = s.snapshot();
        for word in ["forest", "moon", "лес"] {
            assert!(s.handle(Event::Said(word.into())).is_empty());
        }
        if id != unmath::ADDITION_ID {
            assert_ne!(s.snapshot().phase, PhaseKind::Meeting, "{id}");
        }
        assert_eq!(s.snapshot().tasks_completed, before.tasks_completed);
        assert_eq!(s.snapshot().task, before.task);
    }
}

#[test]
fn paused_and_finished_lessons_ignore_late_answers_and_repeated_stop() {
    for id in counting_ids() {
        for finish in [false, true] {
            let mut s = make(&id, config(Language::English));
            start(s.as_mut());
            if finish {
                for _ in 0..32 {
                    if s.snapshot().phase == PhaseKind::Finished {
                        break;
                    }
                    prepare(s.as_mut());
                    if s.snapshot().phase == PhaseKind::Asking {
                        let n = s.snapshot().expected_answer.unwrap();
                        s.handle(Event::Numbers((1..=n).collect()));
                        s.handle(Event::Heard(n));
                    }
                }
                assert_eq!(s.snapshot().phase, PhaseKind::Finished, "{id}");
            } else {
                s.handle(Event::Stop);
            }
            let before = s.snapshot();
            for e in [
                Event::Start,
                Event::Stop,
                Event::Numbers(vec![1, 2, 3, 4, 5]),
                Event::Heard(5),
                Event::RepeatRequested,
                Event::SkipRequested,
                Event::Counted(vec![1]),
                Event::Said("help".into()),
                Event::Said("stop".into()),
                Event::TimerFired(u64::MAX),
            ] {
                assert!(s.handle(e).is_empty(), "{id}");
                let after = s.snapshot();
                assert_eq!(after.phase, before.phase);
                assert_eq!(after.correct, before.correct);
                assert_eq!(after.tasks_completed, before.tasks_completed);
            }
            assert_eq!(s.snapshot().expected_answer, None);
        }
    }
}

#[test]
fn deterministic_event_stress_preserves_score_and_terminal_invariants() {
    for id in ids() {
        for seed in 0..16 {
            let mut rng = SeededGenerator::new(seed);
            let mut s = make(&id, config(Language::English));
            s.handle(Event::Start);
            for _ in 0..256 {
                if matches!(s.snapshot().phase, PhaseKind::Paused | PhaseKind::Finished) {
                    s = make(&id, config(Language::English));
                    s.handle(Event::Start);
                }
                let before = s.snapshot();
                let n = rng.random_int(0, 14);
                let event = match rng.random_int(0, 17) {
                    0 => Event::Start,
                    1 => Event::Stop,
                    2 => Event::RepeatRequested,
                    3 => Event::SkipRequested,
                    4 => Event::NothingHeard,
                    5 => Event::Said("Start again!".into()),
                    6 => Event::Said("Help me.".into()),
                    7 => Event::Table(TableObservation::default()),
                    8 => Event::Table(TableObservation::stable(before.target_count.unwrap_or(n))),
                    9 => Event::Heard(before.expected_answer.unwrap_or(1)),
                    10 => Event::Numbers((1..=before.expected_answer.unwrap_or(3)).collect()),
                    11 => Event::Counted(vec![n]),
                    12 => Event::Sound(n.is_multiple_of(2)),
                    13 => Event::Numbers(vec![0, n, u32::MAX]),
                    14 => Event::Heard(n),
                    15 => Event::TimerFired(u64::MAX),
                    16 => Event::Said("Moon!".into()),
                    _ => Event::Said("I’m tired.".into()),
                };
                let non_answer = !matches!(
                    event,
                    Event::Heard(_) | Event::SkipRequested | Event::Said(_)
                );
                let effects = s.handle(event);
                let after = s.snapshot();
                assert!(after.correct <= after.tasks_completed, "{id}");
                assert!(after.level < after.level_count, "{id}");
                assert!(after.expected_answer.is_none_or(|v| v <= 10));
                if non_answer {
                    assert_eq!(after.correct, before.correct, "{id}: non-answer scored");
                }
                for effect in effects {
                    match effect {
                        Effect::Listen { timeout, .. } => {
                            assert!(timeout.is_finite() && timeout > 0.0)
                        }
                        Effect::SessionEnded(summary) => {
                            assert_eq!(summary.tasks_completed as usize, summary.records.len());
                            assert_eq!(
                                summary.correct as usize,
                                summary.records.iter().filter(|r| r.correct).count()
                            );
                            assert_eq!(summary.paused, after.phase == PhaseKind::Paused);
                            for record in summary.records.iter().filter(|r| r.correct) {
                                assert!(!record.skipped);
                                assert_eq!(record.answers.last().copied(), record.expected);
                                if (id == unmath::ADDITION_ID && !record.task.contains("(total)"))
                                    || (id == unmath::ID && record.task != "Total")
                                    || (id == "unmath-count-to-ten" && record.expected != Some(0))
                                {
                                    assert_eq!(
                                        record.answers,
                                        (1..=record.expected.unwrap()).collect::<Vec<_>>()
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[test]
fn an_impossible_all_complete_checkpoint_is_reset_not_announced_as_new_mastery() {
    for id in ids().into_iter().filter(|id| id != unmath::ID) {
        let mut cfg = config(Language::English);
        cfg.extra
            .insert("unmath.lesson2.one_by_one.v3".into(), "127".into());
        for n in 3..=10 {
            cfg.extra
                .insert(format!("unmath.lesson{n}.adventure.v1"), "63".into());
        }
        let mut s = make(&id, cfg);
        s.handle(Event::Start);
        assert_eq!(s.snapshot().level, 0, "{id}");
        assert_ne!(s.snapshot().phase, PhaseKind::Finished);
    }
}

#[test]
fn russian_raz_and_odin_are_equivalent_in_all_counting_lessons() {
    use un_core::numbers::{NumberParser, NumberWords};
    for id in counting_ids() {
        let mut s = counting(&id, Language::Russian);
        let before = s.snapshot();
        let n = before.expected_answer.unwrap();
        let canonical: Vec<_> = (1..=n)
            .map(|v| NumberWords::spoken(v, Language::Russian))
            .collect();
        let mut alternative = canonical.clone();
        alternative[0] = "раз".into();
        let text = format!("{}, {}", alternative.join(", "), canonical.join(", "));
        let numbers = NumberParser::all(&text, Language::Russian);
        assert_eq!(
            numbers,
            (1..=n).chain(1..=n).collect::<Vec<_>>(),
            "{id}: {text}"
        );
        respond(s.as_mut(), numbers, n);
        assert_eq!(s.snapshot().correct, before.correct + 1, "{id}");
        assert_eq!(
            s.snapshot().tasks_completed,
            before.tasks_completed + 1,
            "{id}"
        );
    }
}
