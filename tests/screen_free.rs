// SPDX-License-Identifier: LGPL-3.0-or-later
//! All child-facing speech is usable without looking at or operating a screen.
use un_core::{language::Language, numbers::tokenize, strategy::*};
fn inspect(effects: Vec<Effect>, id: &str) {
    for effect in effects {
        if let Effect::Say(text) = effect {
            for word in tokenize(&text) {
                assert!(
                    !matches!(
                        word.as_str(),
                        "tap"
                            | "tapping"
                            | "press"
                            | "button"
                            | "buttons"
                            | "screen"
                            | "numberpad"
                            | "grown"
                            | "adult"
                            | "adults"
                            | "forest"
                            | "moon"
                            | "snack"
                            | "snacks"
                            | "picnic"
                            | "ticket"
                            | "tickets"
                            | "passenger"
                            | "passengers"
                            | "pretend"
                            | "rocket"
                            | "bus"
                            | "guest"
                            | "guests"
                            | "treat"
                            | "treats"
                            | "ship"
                            | "ships"
                            | "pods"
                            | "cargo"
                            | "keys"
                            | "key"
                            | "gate"
                            | "gates"
                            | "traveler"
                            | "travelers"
                            | "block"
                            | "blocks"
                            | "chef"
                    ) && ![
                        "нажми",
                        "нажмит",
                        "кноп",
                        "экран",
                        "нажат",
                        "взросл",
                        "угощ",
                        "пикник",
                        "пассаж",
                        "капсул",
                        "светля",
                        "билет",
                        "ракет",
                        "мишк",
                        "инопланет",
                        "пещер",
                        "паром",
                        "повар",
                        "гост",
                        "лагер",
                        "корабл",
                        "блок",
                        "ключ",
                        "путешествен"
                    ]
                    .iter()
                    .any(|prefix| word.starts_with(prefix)),
                    "{id}: {text}"
                );
            }
        }
    }
}
#[test]
fn all_ten_lessons_start_directly_and_use_only_real_toys_without_adult_prompts() {
    let mut registry = StrategyRegistry::default();
    unmath::register(&mut registry);
    for info in registry.infos() {
        for language in [Language::English, Language::Russian] {
            for help in [false, true] {
                let cfg = SessionConfig {
                    language,
                    max_tasks: 100,
                    ..Default::default()
                };
                let mut s = registry.create(&info.id, &cfg).unwrap();
                inspect(s.handle(Event::Start), &info.id);
                assert_ne!(
                    s.snapshot().phase,
                    PhaseKind::Meeting,
                    "{} must start directly",
                    info.id
                );
                let mut helped = false;
                for _ in 0..100 {
                    let snap = s.snapshot();
                    if snap.phase == PhaseKind::Finished {
                        break;
                    }
                    assert_ne!(
                        snap.phase,
                        PhaseKind::Paused,
                        "{} stopped before completion",
                        info.id
                    );
                    if snap.phase == PhaseKind::Performing {
                        inspect(
                            s.handle(Event::Table(TableObservation::stable(
                                snap.target_count.unwrap(),
                            ))),
                            &info.id,
                        );
                    } else if snap.phase == PhaseKind::Asking {
                        inspect(s.handle(Event::RepeatRequested), &info.id);
                        if help && !helped {
                            inspect(s.handle(Event::Said("help".into())), &info.id);
                            helped = true;
                            continue;
                        }
                        let n = s.snapshot().expected_answer.unwrap();
                        inspect(
                            s.handle(Event::Numbers(if n == 0 {
                                vec![0]
                            } else {
                                (1..=n).collect()
                            })),
                            &info.id,
                        );
                        inspect(s.handle(Event::Heard(n)), &info.id);
                    } else {
                        panic!("unexpected phase {:?}", snap.phase);
                    }
                }
                assert_eq!(s.snapshot().phase, PhaseKind::Finished, "{}", info.id);
            }
        }
    }
}
