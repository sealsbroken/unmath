// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::strategy::*;
use unmath::{AddToFive, CountToFive};
fn spoken(effects: &[Effect]) -> String {
    effects
        .iter()
        .filter_map(|e| {
            if let Effect::Say(t) = e {
                Some(t.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
#[test]
fn child_chooses_the_counting_adventure_by_voice() {
    for (choice, vehicle) in [
        (Event::Said("moon".into()), "rocket"),
        (Event::Said("forest".into()), "bus"),
    ] {
        let mut s = CountToFive::new(SessionConfig::default());
        let intro = s.handle(Event::Start);
        assert!(spoken(&intro).contains("You choose"));
        assert_eq!(s.snapshot().phase, PhaseKind::Meeting);
        assert_eq!(s.snapshot().expected_answer, None);
        s.handle(choice);
        let first = s.handle(Event::Table(TableObservation::stable(3)));
        assert!(spoken(&first).contains(vehicle));
        assert_eq!(s.snapshot().tasks_completed, 0); // choosing a destination is not a maths answer
        s.handle(Event::Said("help".into()));
        assert_eq!(s.snapshot().target_count, Some(1));
        assert_eq!(s.snapshot().correct, 0);
        s.handle(Event::Said("I'm tired".into()));
        assert_eq!(s.snapshot().phase, PhaseKind::Paused);
    }
}
#[test]
fn picnic_choice_survives_pause_and_legacy_fact_scores_are_not_reused() {
    let mut config = SessionConfig::default();
    config
        .extra
        .insert("unmath.lesson2.mastered".into(), "2097151".into());
    let mut s = AddToFive::new(config);
    s.handle(Event::Start);
    assert_eq!(s.snapshot().phase, PhaseKind::Meeting);
    s.handle(Event::Said("moon".into()));
    s.handle(Event::Table(TableObservation::stable(1)));
    s.handle(Event::Table(TableObservation::stable(2)));
    s.handle(Event::Numbers(vec![1, 2]));
    s.handle(Event::Heard(2));
    prepare_picnic(&mut s);
    s.handle(Event::Numbers(vec![1, 2, 3]));
    let reward = s.handle(Event::Heard(3));
    assert!(reward.contains(&Effect::Celebrate));
    assert!(spoken(&reward).contains("blanket"));
    assert!(spoken(&prepare_picnic(&mut s)).contains("aliens"));
    let end = s
        .handle(Event::Stop)
        .into_iter()
        .find_map(|e| {
            if let Effect::SessionEnded(s) = e {
                Some(s)
            } else {
                None
            }
        })
        .unwrap();
    let mut config = SessionConfig::default();
    end.apply_remembered(&mut config.extra);
    assert!(!config.extra.contains_key("unmath.lesson2.mastered"));
    let mut resumed = AddToFive::new(config);
    resumed.handle(Event::Start);
    let effects = prepare_picnic(&mut resumed);
    assert!(spoken(&effects).contains("aliens"));
    resumed.handle(Event::Said("help me".into()));
    assert_eq!(resumed.snapshot().phase, PhaseKind::Performing);
    resumed.handle(Event::Said("stop".into()));
    assert_eq!(resumed.snapshot().phase, PhaseKind::Paused);
}

#[test]
fn a_default_length_picnic_finishes_and_help_can_cycle_back_to_success() {
    let mut s = AddToFive::new(SessionConfig::default());
    s.handle(Event::Start);
    s.handle(Event::Heard(1));
    s.handle(Event::Table(TableObservation::stable(1)));
    s.handle(Event::Table(TableObservation::stable(2)));
    s.handle(Event::Numbers(vec![1, 2]));
    s.handle(Event::Heard(2));
    // Ask for help with 1+2; the practice cycle must return to story missions.
    prepare_picnic(&mut s);
    s.handle(Event::Said("help".into()));
    s.handle(Event::Table(TableObservation::stable(1)));
    s.handle(Event::Table(TableObservation::stable(3)));
    s.handle(Event::Numbers(vec![1, 2, 3]));
    s.handle(Event::Heard(3));
    for _ in 0..7 {
        prepare_picnic(&mut s);
        let expected = s.snapshot().expected_answer.unwrap();
        s.handle(Event::Numbers((1..=expected).collect()));
        s.handle(Event::Heard(expected));
    }
    assert_eq!(s.snapshot().phase, PhaseKind::Finished);
    assert_eq!(s.snapshot().tasks_completed, 9); // two guided baskets plus seven missions
}

fn prepare_picnic(s: &mut AddToFive) -> Vec<Effect> {
    let mut effects = vec![];
    for _ in 0..3 {
        if s.snapshot().phase != PhaseKind::Performing {
            break;
        }
        let n = s.snapshot().target_count.unwrap();
        effects.extend(s.handle(Event::Table(TableObservation::stable(n))));
    }
    effects
}
