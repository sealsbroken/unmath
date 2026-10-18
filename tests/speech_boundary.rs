// SPDX-License-Identifier: LGPL-3.0-or-later
//! Cross-crate regression: the real engine must not erase a zero before grading.
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use un_core::{strategy::*, Language};
use un_engine::{
    doubles::{ScriptedSpeechInput, SilentSpeechOutput},
    Engine, EngineOptions,
};
#[test]
fn spoken_zero_prefix_reaches_the_lesson_in_english_and_russian() {
    for (language, zero_count) in [
        (Language::English, "zero one"),
        (Language::Russian, "ноль один"),
    ] {
        let config = SessionConfig {
            language,
            max_tasks: 20,
            ..Default::default()
        };
        let speech = Arc::new(ScriptedSpeechInput::saying(
            vec!["forest".into(), zero_count.into()],
            Duration::from_millis(20),
        ));
        let voice = Arc::new(SilentSpeechOutput::new(false));
        let engine = Engine::new(
            Box::new(unmath::CountToFive::new(config.clone())),
            config,
            Some(speech),
            voice.clone(),
            EngineOptions {
                chime: false,
                log: Some(Arc::new(|_| {})),
                ..Default::default()
            },
        );
        engine.observe_table(TableObservation::stable(1));
        engine.start();
        let deadline = Instant::now() + Duration::from_secs(8);
        while engine.strategy_snapshot().tasks_completed == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        let snapshot = engine.strategy_snapshot();
        assert_eq!(
            snapshot.tasks_completed,
            1,
            "{:?}: {}",
            snapshot,
            voice.spoken_joined()
        );
        assert_eq!(snapshot.correct, 0);
        assert_eq!(snapshot.target_count, Some(1));
        // Give queued feedback a moment to be spoken by the real effect worker.
        let deadline = Instant::now() + Duration::from_secs(2);
        let expected = if language == Language::English {
            "Zero means no toys"
        } else {
            "Ноль означает"
        };
        while !voice.spoken_joined().contains(expected) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(voice.spoken_joined().contains(expected));
        engine.close();
    }
}

#[test]
fn an_explicit_restart_discards_the_zero_prefix_in_the_real_listener() {
    let config = SessionConfig {
        max_tasks: 20,
        ..Default::default()
    };
    let speech = Arc::new(ScriptedSpeechInput::saying(
        vec![
            "forest".into(),
            "zero".into(),
            "start again".into(),
            "one".into(),
        ],
        Duration::from_millis(30),
    ));
    let engine = Engine::new(
        Box::new(unmath::CountToFive::new(config.clone())),
        config,
        Some(speech),
        Arc::new(SilentSpeechOutput::new(false)),
        EngineOptions {
            chime: false,
            log: Some(Arc::new(|_| {})),
            ..Default::default()
        },
    );
    engine.observe_table(TableObservation::stable(1));
    engine.start();
    let deadline = Instant::now() + Duration::from_secs(8);
    while engine.strategy_snapshot().tasks_completed == 0 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let snap = engine.strategy_snapshot();
    assert_eq!(snap.tasks_completed, 1);
    assert_eq!(snap.correct, 1);
    engine.close();
}

#[test]
fn repeated_complete_two_toy_answers_are_accepted_in_single_and_split_utterances() {
    for (language, utterance) in [
        (Language::Russian, "один два"),
        (Language::English, "one two"),
    ] {
        for split in [false, true] {
            let mut config = SessionConfig {
                language,
                max_tasks: 20,
                ..Default::default()
            };
            config
                .extra
                .insert("unmath.lesson1.warmup_three.v1".into(), "2".into());
            let script = if split {
                vec![utterance.to_string(), utterance.to_string()]
            } else {
                vec![format!("{utterance} {utterance}")]
            };
            let speech = Arc::new(ScriptedSpeechInput::saying(
                script,
                Duration::from_millis(20),
            ));
            let voice = Arc::new(SilentSpeechOutput::new(false));
            let engine = Engine::new(
                Box::new(unmath::CountToThree::new(config.clone())),
                config,
                Some(speech),
                voice.clone(),
                EngineOptions {
                    chime: false,
                    log: Some(Arc::new(|_| {})),
                    ..Default::default()
                },
            );
            engine.observe_table(TableObservation::stable(2));
            engine.start();
            let deadline = Instant::now() + Duration::from_secs(8);
            while engine.strategy_snapshot().tasks_completed == 0 && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(10));
            }
            let snapshot = engine.strategy_snapshot();
            let said = voice.spoken_joined();
            engine.close();
            assert_eq!(
                snapshot.tasks_completed, 1,
                "{language:?}, split={split}: {said}"
            );
            assert_eq!(snapshot.correct, 1, "{language:?}, split={split}: {said}");
            assert_eq!(snapshot.target_count, Some(3));
        }
    }
}
