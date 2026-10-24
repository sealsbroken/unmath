// SPDX-License-Identifier: LGPL-3.0-or-later
use un_core::{language::Language, strategy::*};

fn ready(s: &mut dyn Strategy, effects: &mut Vec<Effect>) {
    for _ in 0..5 {
        if s.snapshot().phase != PhaseKind::Performing {
            return;
        }
        effects.extend(s.handle(Event::Table(TableObservation::stable(
            s.snapshot().target_count.unwrap(),
        ))));
    }
    assert_ne!(s.snapshot().phase, PhaseKind::Performing);
}

#[test]
fn an_incorrect_answer_prefaces_the_next_question_once_in_every_lesson() {
    let mut registry = StrategyRegistry::default();
    unmath::register(&mut registry);
    for info in registry.infos() {
        for (language, prefix) in [
            (Language::Russian, "А ты уверен?"),
            (Language::English, "Are you sure?"),
        ] {
            for skipped in [false, true] {
                let mut s = registry
                    .create(
                        &info.id,
                        &SessionConfig {
                            language,
                            max_tasks: 100,
                            ..Default::default()
                        },
                    )
                    .unwrap();
                let mut initial = s.handle(Event::Start);

                if s.snapshot().phase == PhaseKind::Meeting {
                    s.handle(Event::Numbers(vec![1, 2]));
                    s.handle(Event::Heard(2));
                }
                ready(s.as_mut(), &mut initial);
                assert!(!spoken(&initial).iter().any(|line| line.contains(prefix)));
                let before = s.snapshot();
                let mut correction = s.handle(if skipped {
                    Event::SkipRequested
                } else {
                    Event::Heard(99)
                });
                ready(s.as_mut(), &mut correction);
                let prefixes = spoken(&correction)
                    .iter()
                    .filter(|line| line.starts_with(prefix))
                    .count();
                assert_eq!(
                    prefixes,
                    usize::from(!skipped),
                    "{}: {}",
                    info.id,
                    spoken(&correction).join(" ")
                );
                assert_eq!(s.snapshot().level, before.level);
                // A manual repeat doesn't duplicate the pending reassurance prefix.
                let repeated = s.handle(Event::RepeatRequested);
                assert!(!spoken(&repeated).iter().any(|line| line.contains(prefix)));
            }
        }
    }
}
