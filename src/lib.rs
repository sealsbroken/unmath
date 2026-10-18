// SPDX-License-Identifier: LGPL-3.0-or-later
//! Deterministic, on-device counting lessons plugged into unengine's Strategy API.
mod addition;
mod answers;
use answers::{count_feedback, safe_config, valid_count, CountBuffer};
pub mod curriculum;
mod play;
mod russian;
pub use addition::{AddToFive, AddToFiveFactory, ADDITION_ID};
use play::{wants_break, wants_help, wants_restart};

use std::sync::Arc;
use un_core::language::Language;
use un_core::strategy::*;

pub const ID: &str = "unmath-count-to-three";
const PROGRESS: &str = "unmath.lesson1.count_three.v1";
const WARMUP: &str = "unmath.lesson1.warmup_three.v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Question {
    Warmup(u32),
    First,
    Practice(u32),
    Repair(u32),
    Toys,
    Total,
    Final,
}

/// Lesson one: two independent full counts, separated by counting real toys.
/// Only full Numbers sequences are accepted for counting; taps never prove mastery.
pub struct CountToThree {
    config: SessionConfig,
    question: Question,
    phase: PhaseKind,
    table: Option<u32>,
    sequence: CountBuffer,
    first_success: bool,
    resume_after_practice: Question,
    repair_return: Question,
    rechecking: bool,
    retry_preface: bool,
    attempts: u32,
    misses: u32,
    records: Vec<TaskRecord>,
}

impl CountToThree {
    pub fn new(config: SessionConfig) -> Self {
        let config = safe_config(config);
        let first_success = config.extra.get(PROGRESS).is_some_and(|v| v == "true");
        let question = if first_success {
            Question::Toys
        } else {
            match config.extra.get(WARMUP).map(String::as_str) {
                Some("2") => Question::Warmup(2),
                Some("3") => Question::First,
                _ => Question::Warmup(1),
            }
        };
        Self {
            config,
            question,
            phase: PhaseKind::Idle,
            table: None,
            sequence: CountBuffer::default(),
            first_success,
            resume_after_practice: question,
            repair_return: question,
            rechecking: false,
            retry_preface: false,
            attempts: 0,
            misses: 0,
            records: vec![],
        }
    }
    fn target(&self) -> Option<u32> {
        match self.question {
            Question::Warmup(n) | Question::Practice(n) | Question::Repair(n) => Some(n),
            Question::First | Question::Final | Question::Toys | Question::Total => Some(3),
        }
    }
    fn expected(&self) -> u32 {
        self.target().unwrap_or(3)
    }
    fn text(&self, en: &str, ru: &str) -> String {
        match self.config.language {
            Language::English => en,
            Language::Russian => ru,
        }
        .into()
    }
    fn prompt(&self) -> String {
        if self.rechecking && !matches!(self.question, Question::Repair(_) | Question::Practice(_))
        {
            return if self.question == Question::Total {
                self.text(
                    "Remember the last number you said. How many toys are there?",
                    "Вспомни последнее число. Сколько всего игрушек?",
                )
            } else {
                self.text(
                    "Now point to each toy and count them all by yourself.",
                    "Теперь покажи на каждую игрушку и посчитай все игрушки без подсказки.",
                )
            };
        }
        match self.question {
            Question::Repair(_) => self.text(
                "Point to each toy once and say one number for it. Start at one. Stop after the last toy.",
                "Показывай на каждую игрушку по очереди и называй одно число. Начни с одного. После последней игрушки остановись."),
            Question::Warmup(1) => self.text(
                "Here is one toy. Point to it. One! Say one.",
                "Вот одна игрушка. Покажи на неё. Один! Скажи: один.",
            ),
            Question::Warmup(_) => self.text(
                "One more toy! Point to each toy. One, two! Your turn.",
                "Ещё одна игрушка! Покажи на каждую игрушку. Один, два! Теперь ты.",
            ),
            Question::First => self.text(
                "Three toys! Point to each toy. Say one number for each. Start at one.",
                "Три игрушки! Показывай на каждую игрушку по очереди и считай. Начни с одного.",
            ),
            Question::Final => self.text(
                "Move the toys around. Point to each toy and count again!",
                "Покажи на каждую игрушку и посчитай ещё раз!",
            ),
            Question::Total => self.text(
                "How many toys are there altogether?",
                "Сколько всего игрушек?",
            ),
            _ => self.text(
                "Point to each toy. Say one number for each toy. Start at one.",
                "Показывай на каждую игрушку по очереди и считай. Начни с одного.",
            ),
        }
    }
    /// Always keep the opening at one, even when extra toys are visible.
    fn setup_line(&self, n: u32) -> String {
        let toys = match n {
            1 => "одну игрушку",
            2 => "две игрушки",
            _ => "три игрушки",
        };
        self.text(
            &format!(
                "Put {} here. Keep the other toys aside.",
                crate::play::toy_amount(n)
            ),
            &format!("Поставь сюда {toys}. Остальные пока отложи."),
        )
    }
    fn ask(&mut self) -> Vec<Effect> {
        self.sequence.reset();
        if let Some(n) = self.target() {
            if n > self.config.total_objects {
                let mut effects = vec![Effect::Say(self.text(
                    &format!(
                        "We need {n} toys for the next count. We can continue when they are ready."
                    ),
                    "Нам нужна ещё одна игрушка. Продолжим, когда она будет готова.",
                ))];
                effects.extend(self.finish(false));
                return effects;
            }
            if self.table != Some(n) {
                self.phase = PhaseKind::Performing;
                return vec![Effect::StopListening, Effect::Say(self.setup_line(n))];
            }
        }
        self.phase = PhaseKind::Asking;
        let mut effects = vec![];
        if let Question::Practice(n) | Question::Repair(n) = self.question {
            // Model only a verified set. Setup is requested before modeling otherwise.
            if self.table == Some(n) {
                effects.push(Effect::HighlightCount(n));
                let words = match self.config.language {
                    Language::English => ["one", "two", "three"],
                    Language::Russian => ["один", "два", "три"],
                };
                effects.push(Effect::Say(format!(
                    "{}. {}",
                    words[..n as usize].join(", "),
                    self.text("Now try by yourself.", "Теперь твоя очередь.")
                )));
            }
        }
        let question = Effect::Say(self.prompt());
        let question =
            crate::play::preface_retry(&mut self.retry_preface, self.config.language, question);
        effects.extend(vec![
            question,
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
            WARMUP.into(),
            if mastered {
                String::new()
            } else {
                match if matches!(self.question, Question::Repair(_)) {
                    self.repair_return
                } else {
                    self.question
                } {
                    Question::Warmup(2) => "2",
                    Question::First => "3",
                    _ => "1",
                }
                .into()
            },
        );
        summary
            .remember
            .insert("unmath.lesson1.place".into(), String::new());
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
                "You counted to three by yourself! Well done!",
                "Отлично! Мы досчитали до трёх!",
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
                answer == Some(3)
            } else {
                valid_count(&numbers, expected, answer)
            };
        let feedback = (!correct && self.question != Question::Total)
            .then(|| count_feedback(&numbers, expected, self.config.language));
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
        if correct {
            if let Question::Warmup(n) = self.question {
                self.rechecking = false;
                self.question = if n == 1 {
                    Question::Warmup(2)
                } else {
                    Question::First
                };
                self.misses = 0;
                if self.attempts >= self.config.max_tasks {
                    return self.finish(false);
                }
                let mut effects = vec![
                    Effect::StopListening,
                    Effect::Say(self.text("Well done!", "Отлично!")),
                ];
                effects.extend(self.ask());
                return effects;
            }
        }
        if correct && self.question == Question::First {
            self.first_success = true;
        }
        if self.attempts >= self.config.max_tasks.max(1) {
            return self.finish(false);
        }
        let mut effects = vec![Effect::StopListening];
        effects.extend(feedback);
        if correct {
            self.misses = 0;
            self.rechecking = false;
            effects.push(Effect::Say(self.text("Well done!", "Отлично!")));
            self.question = match self.question {
                Question::Warmup(_) => unreachable!("warm-up success advances above"),
                Question::First => {
                    self.first_success = true;
                    Question::Toys
                }
                Question::Practice(n) if n < 3 => Question::Practice(n + 1),
                Question::Practice(_) => self.resume_after_practice,
                Question::Repair(_) => {
                    self.rechecking = true;
                    self.repair_return
                }
                Question::Toys => Question::Total,
                Question::Total => {
                    effects.push(Effect::Say(self.text(
                        "Move the toys around. Then count again.",
                        "Передвинь игрушки. Потом посчитаем ещё раз.",
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
            self.retry_preface = !skipped;
            // Teach one-to-one counting with this group; never shrink it or
            // award an independent success for the tutor-modeled count.
            if !matches!(self.question, Question::Repair(_) | Question::Practice(_)) {
                self.repair_return = self.question;
                self.rechecking = false;
                self.question = Question::Repair(expected);
            }
        }
        effects.extend(self.ask());
        effects
    }
}

impl Strategy for CountToThree {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        if matches!(&event, Event::Heard(n) if self.sequence.ignore_discarded_tail(*n)) {
            return vec![];
        }
        if matches!(&event, Event::Said(words) if wants_restart(words)) {
            return self.handle(Event::RepeatRequested);
        }
        match event {
            Event::Start if self.phase == PhaseKind::Idle => {
                if self.config.max_tasks == 0 {
                    return self.finish(false);
                }
                if self.config.total_objects < 1 {
                    let mut effects = vec![Effect::Say(self.text(
                        "We need one toy to begin. We can count when it is ready.",
                        "Для начала нужна одна игрушка. Посчитаем, когда она будет готова.",
                    ))];
                    effects.extend(self.finish(false));
                    return effects;
                }
                self.ask()
            }
            Event::Said(words)
                if wants_break(&words)
                    && matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing) =>
            {
                self.finish(false)
            }
            Event::Said(words) if wants_help(&words) && self.phase == PhaseKind::Asking => {
                self.sequence.reset();
                if !matches!(self.question, Question::Practice(_)) {
                    self.resume_after_practice = self.question;
                }
                self.question = Question::Practice(1);
                self.ask()
            }
            Event::Stop if matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing) => {
                self.finish(false)
            }
            Event::Table(t) => {
                // A moving table that still shows the settled count (a hand pointing at the
                // toys) changes nothing; only a lost or different count does. Pointing used to
                // restart the question on every frame.
                if !t.is_stable && t.count.is_some() && t.count == self.table {
                    return vec![];
                }
                self.table = if t.is_stable { t.count } else { None };
                if matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing)
                    && self.target().is_some()
                {
                    if self.phase == PhaseKind::Performing && self.table == self.target() {
                        return self.ask();
                    }
                    if self.phase == PhaseKind::Asking && self.table != self.target() {
                        self.sequence.reset();
                        self.phase = PhaseKind::Performing;
                        return vec![Effect::StopListening];
                    }
                }
                vec![]
            }
            // Judge on Heard, after caching Numbers: the engine sends this pair in order.
            // This prevents the old count's trailing Heard from answering the next question.
            Event::Numbers(ns) if self.phase == PhaseKind::Asking => {
                self.sequence.set(ns);
                vec![]
            }
            Event::Heard(n) if self.phase == PhaseKind::Asking => self.answer(Some(n), false),
            Event::NothingHeard if self.phase == PhaseKind::Asking => {
                self.sequence.reset();
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
            Event::SkipRequested if self.phase == PhaseKind::Asking => self.answer(None, true),
            Event::SkipRequested if self.phase == PhaseKind::Performing => self.finish(false),
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
            target_count: matches!(self.phase, PhaseKind::Asking | PhaseKind::Performing)
                .then(|| self.expected()),
            expected_answer: (self.phase == PhaseKind::Asking).then(|| self.expected()),
        }
    }
}

#[derive(Default)]
pub struct CountToThreeFactory;
impl StrategyFactory for CountToThreeFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo {
            id: ID.into(),
            title: "Lesson 1: count three toys".into(),
            description: "Start with one toy and learn to count three toys one by one.".into(),
            languages: vec![Language::English, Language::Russian],
            license: "LGPL-3.0-or-later".into(),
        }
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(CountToThree::new(config.clone()))
    }
}
pub fn register(registry: &mut StrategyRegistry) {
    registry.register(Arc::new(CountToThreeFactory));
    registry.register(Arc::new(AddToFiveFactory));
    curriculum::register(registry);
}
inventory::submit! { LinkedStrategy(|| Arc::new(CountToThreeFactory)) }

/// Compatibility name; lesson one now teaches counting to three.
pub type CountToFive = CountToThree;
pub use CountToThreeFactory as CountToFiveFactory;
