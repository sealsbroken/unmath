// SPDX-License-Identifier: LGPL-3.0-or-later
//! Lesson two, the bus («Lesson 2_3»): warm-up, 1 → 3 (how many got on), 3 + 2 (how many ride).
use un_core::{language::Language, numbers::tokenize, strategy::*};
use unmath::AddToFive;

const HOLD: u64 = 201;
const REMINDER: u64 = 200;

fn lesson(language: Language) -> AddToFive {
    AddToFive::new(SessionConfig {
        language,
        ..Default::default()
    })
}
fn text(es: &[Effect]) -> String {
    spoken(es).join(" ")
}
fn table(s: &mut AddToFive, n: u32) -> Vec<Effect> {
    s.handle(Event::Table(TableObservation::stable(n)))
}
/// The chips are right and stay right through the silent pause.
fn place(s: &mut AddToFive, n: u32) -> Vec<Effect> {
    let mut es = table(s, n);
    es.extend(s.handle(Event::TimerFired(HOLD)));
    es
}
fn holds(es: &[Effect]) -> Vec<f64> {
    es.iter()
        .filter_map(|e| match e {
            Effect::StartTimer {
                token: HOLD,
                seconds,
            } => Some(*seconds),
            _ => None,
        })
        .collect()
}
fn summary(es: &[Effect]) -> SessionSummary {
    es.iter()
        .find_map(|e| match e {
            Effect::SessionEnded(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap()
}
/// Warm-up done, one chip placed: the goal of three is set.
fn at_goal(s: &mut AddToFive) {
    s.handle(Event::Start);
    s.handle(Event::Said("поехали".into()));
    place(s, 1);
}

#[test]
fn the_whole_ride_in_both_languages() {
    for (language, lines) in [
        (
            Language::Russian,
            [
                "Дрррм! Наш автобус завёл мотор",
                "Отлично, едем! Наш автобус — это лист бумаги на столе.",
                "В автобусе сидит один пассажир. Положи одну фишку.",
                "Сделай так, чтобы их стало три.",
                "сколько именно пассажиров сейчас зашло в автобус?",
                "Точно, два!",
                "Ты уже знаешь, что сейчас в автобусе едут три пассажира.",
                "На следующей остановке зашли ещё два пассажира. Добавь две фишки.",
                "Автобус поехал! Сколько теперь пассажиров едет в автобусе?",
                "Пять! Все на месте.",
            ],
        ),
        (
            Language::English,
            [
                "Vroom! Our bus has started its engine",
                "Great, off we go!",
                "Put one chip on the paper.",
                "Make it three.",
                "how many passengers just got on the bus?",
                "Exactly, two!",
                "three passengers are riding in the bus now",
                "Add two chips.",
                "How many passengers are riding in the bus now?",
                "Five! Everyone is here.",
            ],
        ),
    ] {
        let mut s = lesson(language);
        let mut said = text(&s.handle(Event::Start));
        assert_eq!(s.snapshot().phase, PhaseKind::Meeting);
        said += &text(&s.handle(Event::Said("go".into())));
        assert_eq!(s.snapshot().target_count, Some(1));
        said += &text(&place(&mut s, 1));
        assert_eq!(s.snapshot().target_count, Some(3));
        said += &text(&place(&mut s, 3));
        assert_eq!(s.snapshot().expected_answer, Some(2));
        said += &text(&s.handle(Event::Heard(2)));
        assert_eq!(s.snapshot().target_count, Some(5));
        assert_eq!(s.snapshot().level, 1);
        said += &text(&place(&mut s, 5));
        assert_eq!(s.snapshot().expected_answer, Some(5));
        let end = s.handle(Event::Heard(5));
        said += &text(&end);
        for line in lines {
            assert!(said.contains(line), "{line:?} missing from {said}");
        }
        let end = summary(&end);
        assert!(!end.paused);
        assert_eq!((end.tasks_completed, end.correct), (2, 2));
        assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    }
}

#[test]
fn warm_up_goes_on_after_any_speech_or_seven_seconds_of_silence() {
    for event in [
        Event::Said("бип".into()),
        Event::Heard(7),
        Event::NothingHeard,
    ] {
        let mut s = lesson(Language::Russian);
        let es = s.handle(Event::Start);
        assert!(es.iter().any(
            |e| matches!(e, Effect::Listen { mode: ListenMode::Voice, timeout } if *timeout == 7.0)
        ));
        s.handle(event);
        assert_eq!(s.snapshot().target_count, Some(1));
        assert_eq!(s.snapshot().tasks_completed, 0, "the warm-up is not scored");
    }
}

#[test]
fn chips_wait_out_a_silent_pause_and_a_moved_chip_cancels_it() {
    let mut s = lesson(Language::English);
    s.handle(Event::Start);
    s.handle(Event::NothingHeard);
    // 3 s for the first chip, 4 s after adding.
    assert_eq!(holds(&table(&mut s, 1)), vec![3.0]);
    assert!(
        table(&mut s, 1).is_empty(),
        "the same frame again changes nothing"
    );
    let es = table(&mut s, 2);
    assert!(es.iter().any(|e| matches!(e, Effect::CancelTimer(HOLD))));
    assert!(s.handle(Event::TimerFired(HOLD)).is_empty());
    assert_eq!(s.snapshot().target_count, Some(1));
    place(&mut s, 1);
    // The goal: wrong amounts, unstable frames and hands wait silently.
    for n in [2, 4] {
        assert!(spoken(&table(&mut s, n)).is_empty());
    }
    assert!(s
        .handle(Event::Table(TableObservation::default()))
        .is_empty());
    assert_eq!(holds(&table(&mut s, 3)), vec![4.0]);
    s.handle(Event::Table(TableObservation::default()));
    assert!(s.handle(Event::TimerFired(HOLD)).is_empty());
    assert_eq!(s.snapshot().phase, PhaseKind::Performing);
    place(&mut s, 3);
    assert_eq!(s.snapshot().phase, PhaseKind::Asking);
}

#[test]
fn a_wrong_or_missing_answer_gets_the_hint_then_the_ride_goes_on() {
    for (first, second) in [
        (Event::Heard(3), Event::Heard(1)),
        (Event::NothingHeard, Event::Heard(2)),
        (Event::Said("help".into()), Event::NothingHeard),
    ] {
        let mut s = lesson(Language::Russian);
        at_goal(&mut s);
        place(&mut s, 3);
        let hint = text(&s.handle(first));
        assert!(
            hint.contains("Был один... Стало три. Сколько ты добавил?"),
            "{hint}"
        );
        assert_eq!(s.snapshot().expected_answer, Some(2));
        let on = text(&s.handle(second.clone()));
        assert!(on.contains("Двигаемся дальше."));
        assert_eq!(on.contains("Точно, два!"), second == Event::Heard(2));
        assert_eq!(s.snapshot().target_count, Some(5));
        place(&mut s, 5);
        let hint = text(&s.handle(Event::Heard(4)));
        assert!(
            hint.contains("у нас было три, и пришло ещё два... Это..."),
            "{hint}"
        );
        let end = s.handle(Event::NothingHeard);
        assert!(text(&end).contains("Пять! Все на месте."));
        assert_eq!(s.snapshot().phase, PhaseKind::Finished);
        assert!(summary(&end).records.iter().any(|r| !r.correct));
    }
}

#[test]
fn too_many_chips_in_round_two_take_the_new_ones_off_then_add_again() {
    let mut s = lesson(Language::Russian);
    at_goal(&mut s);
    place(&mut s, 3);
    s.handle(Event::Heard(2));
    let undo = text(&table(&mut s, 6));
    assert!(
        undo.contains("Ой, у меня запотело стекло. Убери со стола новые фишки"),
        "{undo}"
    );
    assert_eq!(s.snapshot().target_count, Some(3));
    assert!(spoken(&table(&mut s, 5)).is_empty(), "five is not three");
    let again = text(&table(&mut s, 3));
    assert!(again.contains("Добавь две фишки."), "{again}");
    assert_eq!(s.snapshot().target_count, Some(5));
    // An overshoot during the pause undoes too.
    table(&mut s, 5);
    table(&mut s, 7);
    assert_eq!(s.snapshot().target_count, Some(3));
    table(&mut s, 3);
    place(&mut s, 5);
    assert_eq!(s.snapshot().expected_answer, Some(5));
    assert_eq!(
        s.snapshot().tasks_completed,
        1,
        "chip corrections are not scored"
    );
}

#[test]
fn counting_aloud_to_five_answers_the_last_question() {
    let mut s = lesson(Language::English);
    at_goal(&mut s);
    place(&mut s, 3);
    s.handle(Event::Heard(2));
    let es = place(&mut s, 5);
    assert_eq!(listens(&es), vec![ListenMode::Counting]);
    s.handle(Event::Numbers(vec![1, 2, 3, 4, 5]));
    let end = s.handle(Event::Heard(5));
    assert!(summary(&end).records.iter().all(|r| r.correct));
}

#[test]
fn break_skip_and_stop_pause_and_a_paused_ride_ignores_late_events() {
    for event in [
        Event::Said("я устал".into()),
        Event::SkipRequested,
        Event::Stop,
    ] {
        let mut s = lesson(Language::Russian);
        at_goal(&mut s);
        let end = s.handle(event);
        assert!(summary(&end).paused);
        assert_eq!(s.snapshot().phase, PhaseKind::Paused);
        for late in [Event::Heard(2), Event::TimerFired(HOLD), Event::Start] {
            assert!(s.handle(late).is_empty());
        }
    }
}

#[test]
fn repeat_and_reminders_replay_the_instruction_without_scoring() {
    let mut s = lesson(Language::Russian);
    at_goal(&mut s);
    let again = text(&s.handle(Event::Said("повтори".into())));
    assert!(again.contains("Сделай так, чтобы их стало три."));
    let reminder = text(&s.handle(Event::TimerFired(REMINDER)));
    assert!(reminder.contains("Сделай так, чтобы их стало три."));
    assert_eq!(s.snapshot().tasks_completed, 0);
}

#[test]
fn fewer_than_five_toys_or_no_answers_pause_at_once_and_old_progress_is_cleared() {
    let mut s = AddToFive::new(SessionConfig {
        total_objects: 4,
        ..Default::default()
    });
    assert!(summary(&s.handle(Event::Start)).paused);
    let mut config = SessionConfig::default();
    config
        .extra
        .insert("unmath.lesson2.graph.v4.index".into(), "6".into());
    let mut s = AddToFive::new(config.clone());
    s.handle(Event::Start);
    assert_eq!(
        s.snapshot().phase,
        PhaseKind::Meeting,
        "every ride starts with the bus"
    );
    summary(&s.handle(Event::Stop)).apply_remembered(&mut config.extra);
    assert!(!config.extra.contains_key("unmath.lesson2.graph.v4.index"));
}

#[test]
fn nothing_asks_for_a_screen_or_a_grown_up() {
    for language in [Language::English, Language::Russian] {
        let mut s = lesson(language);
        let mut es = s.handle(Event::Start);
        es.extend(s.handle(Event::NothingHeard));
        es.extend(place(&mut s, 1));
        es.extend(place(&mut s, 3));
        es.extend(s.handle(Event::Heard(9)));
        es.extend(s.handle(Event::Heard(9)));
        es.extend(table(&mut s, 6));
        es.extend(table(&mut s, 3));
        es.extend(place(&mut s, 5));
        es.extend(s.handle(Event::Heard(9)));
        es.extend(s.handle(Event::Heard(9)));
        for line in spoken(&es) {
            for word in tokenize(line) {
                assert!(
                    !["tap", "press", "button", "screen", "grown", "adult"]
                        .contains(&word.as_str())
                        && !["нажм", "кноп", "экран", "взросл"]
                            .iter()
                            .any(|p| word.starts_with(p)),
                    "{line}"
                );
            }
        }
    }
}

/// Applies timer effects the way the engine does: a timer starts once, and a cancel must find
/// it running (otherwise the engine drops its next start).
fn timers(running: &mut std::collections::BTreeSet<u64>, es: Vec<Effect>) {
    for e in es {
        match e {
            Effect::StartTimer { token, .. } => assert!(running.insert(token), "{token} twice"),
            Effect::CancelTimer(token) => assert!(running.remove(&token), "{token} is not running"),
            _ => {}
        }
    }
}
#[test]
fn timers_are_only_cancelled_while_running() {
    let mut on = std::collections::BTreeSet::new();
    let mut s = lesson(Language::English);
    timers(&mut on, s.handle(Event::Start));
    timers(&mut on, s.handle(Event::NothingHeard));
    timers(&mut on, table(&mut s, 2));
    on.remove(&REMINDER);
    timers(&mut on, s.handle(Event::TimerFired(REMINDER)));
    timers(&mut on, table(&mut s, 1));
    timers(&mut on, table(&mut s, 0));
    timers(&mut on, table(&mut s, 1));
    on.remove(&HOLD);
    timers(&mut on, s.handle(Event::TimerFired(HOLD)));
    timers(&mut on, table(&mut s, 3));
    on.remove(&HOLD);
    timers(&mut on, s.handle(Event::TimerFired(HOLD)));
    timers(&mut on, s.handle(Event::Heard(2)));
    timers(&mut on, table(&mut s, 6));
    timers(&mut on, table(&mut s, 3));
    timers(&mut on, table(&mut s, 5));
    timers(&mut on, s.handle(Event::Stop));
    assert!(on.is_empty(), "{on:?} left running");
}
