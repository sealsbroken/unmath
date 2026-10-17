// SPDX-License-Identifier: LGPL-3.0-or-later
//! Deterministic, on-device counting lessons plugged into unengine's Strategy API.
mod addition;
pub mod curriculum;
mod play;
pub use addition::{AddToFive, AddToFiveFactory, ADDITION_ID};
use play::{invitation, wants_break, wants_help, Place};

use std::sync::Arc;
use un_core::language::Language;
use un_core::strategy::*;

pub const ID: &str = "unmath-count-to-five";
const PROGRESS: &str = "unmath.lesson1.one_by_one.v2";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Question {
    First,
    Practice(u32),
    Toys,
    Total,
    Final,
}

/// Lesson one: two independent full counts, separated by counting real toys.
/// Only full Numbers sequences are accepted for counting; taps never prove mastery.
pub struct CountToFive {
    config: SessionConfig,
    place: Place,
    question: Question,
    phase: PhaseKind,
    table: Option<u32>,
    sequence: Option<Vec<u32>>,
    first_success: bool,
    attempts: u32,
    misses: u32,
    records: Vec<TaskRecord>,
}

impl CountToFive {
    pub fn new(config: SessionConfig) -> Self {
        let first_success = config.extra.get(PROGRESS).is_some_and(|v| v == "true");
        let place = config
            .extra
            .get("unmath.lesson1.place")
            .and_then(|s| Place::from_words(s))
            .unwrap_or_default();
        Self {
            place,
            config,
            question: if first_success {
                Question::Toys
            } else {
                Question::First
            },
            phase: PhaseKind::Idle,
            table: None,
            sequence: None,
            first_success,
            attempts: 0,
            misses: 0,
            records: vec![],
        }
    }
    fn target(&self) -> Option<u32> {
        match self.question {
            Question::Practice(n) => Some(n),
            Question::First | Question::Final | Question::Toys | Question::Total => Some(5),
        }
    }
    fn expected(&self) -> u32 {
        self.target().unwrap_or(5)
    }
    fn text(&self, en: &str, ru: &str) -> String {
        match self.config.language {
            Language::English => en,
            Language::Russian => ru,
        }
        .into()
    }
    fn prompt(&self) -> String {
        let vehicle = self.place.vehicle(self.config.language);
        match self.question {
            Question::First => self.text(
                &format!("You're the captain of our {vehicle}! Point to one toy passenger at a time and say one number for each to open the doors. Start at one and keep going until every toy has a number."),
                &format!("Ты капитан! Открой {vehicle}: показывай на игрушечных пассажиров по одному и называй по одному числу для каждого. Начни с одного и считай, пока у каждой игрушки не будет числа.")),
            Question::Final => self.text(
                "Everyone is aboard! Point to each toy passenger, one by one, and say one number for each to start our adventure.",
                "Все на борту! Показывай на игрушечных пассажиров по одному и называй по одному числу для каждого, чтобы отправиться в путь!"),
            Question::Total => self.text(
                "How many passengers are coming to our picnic?",
                "Сколько пассажиров едет на наш пикник?"),
            _ => self.text(
                "Ticket check! Point to the toys one by one. Say one number for each toy, starting at one. Keep going until every toy has a ticket.",
                "Проверяем билеты! Показывай на игрушки по одной. Называй по одному числу для каждой, начиная с одного. Считай, пока у всех не будет билета."),
        }
    }
    fn ask(&mut self) -> Vec<Effect> {
        self.sequence = None;
        if let Some(n) = self.target() {
            if self.table != Some(n) {
                self.phase = PhaseKind::Performing;
                return vec![Effect::StopListening, Effect::Say(match self.config.language {
                    Language::English => format!("Our next stop needs {n} toy passengers! Can you put them on the table? Move your hands away to check the tickets. A grown-up can help."),
                    Language::Russian => format!("На остановке ждут {n} игрушечных пассажиров! Поставь их на стол и убери руки, чтобы проверить билеты. Взрослый может помочь."),
                })];
            }
        }
        self.phase = PhaseKind::Asking;
        let mut effects = vec![];
        if let Question::Practice(n) = self.question {
            // Model only a verified set. Setup is requested before modeling otherwise.
            if self.table == Some(n) {
                effects.push(Effect::HighlightCount(n));
                let words = match self.config.language {
                    Language::English => ["one", "two", "three", "four", "five"],
                    Language::Russian => ["один", "два", "три", "четыре", "пять"],
                };
                effects.push(Effect::Say(format!(
                    "{}. {}",
                    words[..n as usize].join(", "),
                    self.text("Now try by yourself.", "Теперь попробуй самостоятельно.")
                )));
            }
        }
        effects.extend(vec![
            Effect::Say(self.prompt()),
            Effect::listen(
                if self.question == Question::Total {
                    ListenMode::Voice
                } else {
                    ListenMode::Counting
                },
                self.config.answer_timeout,
            ),
        ]);
        effects
    }
    fn finish(&mut self, mastered: bool) -> Vec<Effect> {
        self.phase = if mastered {
            PhaseKind::Finished
        } else {
            PhaseKind::Paused
        };
        let mut summary = SessionSummary {
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            final_level: u32::from(mastered),
            records: self.records.clone(),
            paused: !mastered,
            ..Default::default()
        };
        summary
            .remember
            .insert("unmath.lesson1.first_success".into(), String::new());
        summary.remember.insert(
            "unmath.lesson1.place".into(),
            if mastered {
                String::new()
            } else {
                self.place.saved().into()
            },
        );
        summary.remember.insert(
            PROGRESS.into(),
            if mastered {
                String::new()
            } else {
                self.first_success.to_string()
            },
        );
        let mut effects = vec![Effect::StopListening];
        if mastered {
            effects.push(Effect::Celebrate);
        }
        effects.push(Effect::Say(if mastered {
            self.text(
                "You counted to five by yourself! Everyone has a ticket. Off to our picnic!",
                "Ты самостоятельно посчитал до пяти! У всех есть билеты. Едем на пикник!",
            )
        } else {
            self.text(
                "Let's take a break. We can count again next time.",
                "Давай отдохнём. В следующий раз посчитаем ещё.",
            )
        }));
        effects.push(Effect::SessionEnded(summary));
        effects
    }
    fn answer(&mut self, answer: Option<u32>, skipped: bool) -> Vec<Effect> {
        let numbers = self.sequence.take().unwrap_or_default();
        let expected = self.expected();
        let correct = !skipped
            && if self.question == Question::Total {
                answer == Some(5)
            } else {
                numbers == (1..=expected).collect::<Vec<_>>()
            };
        self.records.push(TaskRecord {
            task: format!("{:?}", self.question),
            answers: if self.question == Question::Total {
                answer.into_iter().collect()
            } else {
                numbers
            },
            expected: Some(expected),
            correct,
            skipped,
        });
        self.attempts += 1;
        if correct && self.question == Question::Final && self.first_success {
            return self.finish(true);
        }
        if correct && self.question == Question::First {
            self.first_success = true;
        }
        if self.attempts >= self.config.max_tasks.max(1) {
            return self.finish(false);
        }
        let mut effects = vec![Effect::StopListening];
        if correct {
            self.misses = 0;
            effects.push(Effect::Say(self.place.cheer(self.config.language).into()));
            self.question = match self.question {
                Question::First => {
                    self.first_success = true;
                    Question::Toys
                }
                Question::Practice(n) if n < 5 => Question::Practice(n + 1),
                Question::Practice(_) => Question::First,
                Question::Toys => Question::Total,
                Question::Total => {
                    effects.push(Effect::Say(self.text(
                        "Let the five passengers choose new seats! Move the toys around, then we can start the engine.",
                        "Пусть пять пассажиров выберут новые места! Передвинь игрушки, а потом заведём мотор.",
                    )));
                    Question::Final
                }
                Question::Final => Question::First,
            };
        } else {
            self.misses += 1;
            if self.misses >= 3 {
                return self.finish(false);
            }
            if self.question == Question::Total {
                self.question = Question::Toys;
            } else {
                let n = match self.question {
                    Question::Practice(n) if self.misses >= 2 => n.saturating_sub(1).max(1),
                    Question::Practice(n) => n,
                    _ => 1,
                };
                self.question = Question::Practice(n);
            }
        }
        effects.extend(self.ask());
        effects
    }
}

impl Strategy for CountToFive {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        match event {
            Event::Start if self.phase == PhaseKind::Idle => {
                if self.first_success {
                    self.ask()
                } else {
                    self.phase = PhaseKind::Meeting;
                    invitation(&self.config)
                }
            }
            Event::Heard(n) if self.phase == PhaseKind::Meeting => {
                if let Some(place) = Place::from_number(n) {
                    self.place = place;
                    self.misses = 0;
                    self.ask()
                } else {
                    invitation(&self.config)
                }
            }
            Event::Said(words)
                if wants_break(&words)
                    && matches!(
                        self.phase,
                        PhaseKind::Meeting | PhaseKind::Asking | PhaseKind::Performing
                    ) =>
            {
                self.finish(false)
            }
            Event::Said(words) if wants_help(&words) && self.phase == PhaseKind::Asking => {
                self.sequence = None;
                self.question = Question::Practice(1);
                self.ask()
            }
            Event::Said(words) if self.phase == PhaseKind::Meeting => {
                if let Some(place) = Place::from_words(&words) {
                    self.place = place;
                    self.misses = 0;
                    self.ask()
                } else {
                    invitation(&self.config)
                }
            }
            Event::RepeatRequested if self.phase == PhaseKind::Meeting => invitation(&self.config),
            Event::NothingHeard if self.phase == PhaseKind::Meeting => {
                self.misses += 1;
                if self.misses >= 3 {
                    self.finish(false)
                } else {
                    invitation(&self.config)
                }
            }
            Event::SkipRequested if self.phase == PhaseKind::Meeting => self.ask(),
            Event::Stop
                if matches!(
                    self.phase,
                    PhaseKind::Meeting | PhaseKind::Asking | PhaseKind::Performing
                ) =>
            {
                self.finish(false)
            }
            Event::Table(t) => {
                self.table = if t.is_stable { t.count } else { None };
                if matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing)
                    && self.target().is_some()
                {
                    if self.phase == PhaseKind::Performing && self.table == self.target() {
                        return self.ask();
                    }
                    if self.phase == PhaseKind::Asking && self.table != self.target() {
                        self.sequence = None;
                        self.phase = PhaseKind::Performing;
                        return vec![Effect::StopListening];
                    }
                }
                vec![]
            }
            // Judge on Heard, after caching Numbers: the engine sends this pair in order.
            // This prevents the old count's trailing Heard from answering the next question.
            Event::Numbers(ns) if self.phase == PhaseKind::Asking => {
                self.sequence = Some(ns);
                vec![]
            }
            Event::Heard(n) if self.phase == PhaseKind::Asking => self.answer(Some(n), false),
            Event::NothingHeard if self.phase == PhaseKind::Asking => {
                self.sequence = None;
                self.misses += 1;
                if self.misses >= 3 {
                    self.finish(false)
                } else {
                    self.ask()
                }
            }
            Event::RepeatRequested
                if matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing) =>
            {
                self.ask()
            }
            Event::SkipRequested
                if matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing) =>
            {
                self.answer(None, true)
            }
            _ => vec![],
        }
    }
    fn snapshot(&self) -> StrategySnapshot {
        StrategySnapshot {
            phase: self.phase,
            level: u32::from(self.phase == PhaseKind::Finished),
            level_count: 2,
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            task: Some(format!("Lesson 1: {:?}", self.question)),
            target_count: self.target(),
            expected_answer: (self.phase == PhaseKind::Asking).then(|| self.expected()),
        }
    }
}

#[derive(Default)]
pub struct CountToFiveFactory;
impl StrategyFactory for CountToFiveFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo {
            id: ID.into(),
            title: "Lesson 1: five passengers".into(),
            description: "Captain a bus or rocket, check toy tickets, and count five passengers."
                .into(),
            languages: vec![Language::English, Language::Russian],
            license: "LGPL-3.0-or-later".into(),
        }
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(CountToFive::new(config.clone()))
    }
}
pub fn register(registry: &mut StrategyRegistry) {
    registry.register(Arc::new(CountToFiveFactory));
    registry.register(Arc::new(AddToFiveFactory));
    curriculum::register(registry);
}
inventory::submit! { LinkedStrategy(|| Arc::new(CountToFiveFactory)) }
