// SPDX-License-Identifier: LGPL-3.0-or-later
//! Lesson two follows the supplied eight-example graph, with bounded correction paths.
use crate::answers::{count_feedback, safe_config, saved_mask, valid_count, CountBuffer};
use crate::play::{toy_amount, wants_break, wants_help, wants_restart};
use crate::russian::{toy_group, toys};
use std::sync::Arc;
use un_core::{language::Language, numbers::NumberWords, strategy::*};

pub const ADDITION_ID: &str = "unmath-add-to-five";
const PROGRESS: &str = "unmath.lesson2.graph.v4.index";
const OLD_PROGRESS: &str = "unmath.lesson2.one_by_one.v3";
const FACTS: [(u32, u32); 8] = [
    (1, 1),
    (1, 2),
    (2, 2),
    (2, 3),
    (3, 1),
    (1, 4),
    (3, 0),
    (1, 1),
];
const SETUP_REMINDER: u64 = 200;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Idle,
    Link,
    Base,
    Add,
    Count,
    Total,
    Guided,
    Final,
    Paused,
    Finished,
}

pub struct AddToFive {
    config: SessionConfig,
    node: Node,
    index: usize,
    table: Option<u32>,
    sequence: CountBuffer,
    link_retry: bool,
    undo_add: bool,
    retry_preface: bool,
    /// Resume the same spoken question after a camera interruption.
    interrupted: Option<Node>,
    last_setup_count: Option<u32>,
    records: Vec<TaskRecord>,
}
impl AddToFive {
    pub fn new(config: SessionConfig) -> Self {
        let config = safe_config(config);
        let index = config
            .extra
            .get(PROGRESS)
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|n| *n < FACTS.len())
            .unwrap_or_else(|| {
                // Keep the contiguous completed prefix from the previous seven-fact course.
                let mask = saved_mask(config.extra.get(OLD_PROGRESS), 127);
                if mask == 0 {
                    return 0;
                }
                let mut index = 1; // the previous opening demonstration was already completed
                while index < 7 && mask & (1 << index) != 0 {
                    index += 1;
                }
                index
            });
        Self {
            config,
            node: Node::Idle,
            index,
            table: None,
            sequence: CountBuffer::default(),
            link_retry: false,
            undo_add: false,
            retry_preface: false,
            interrupted: None,
            last_setup_count: None,
            records: vec![],
        }
    }
    fn fact(&self) -> (u32, u32) {
        FACTS[self.index.min(FACTS.len() - 1)]
    }
    fn sum(&self) -> u32 {
        let (a, b) = self.fact();
        a + b
    }
    fn say(&self, en: impl Into<String>, ru: impl Into<String>) -> Effect {
        Effect::Say(match self.config.language {
            Language::English => en.into(),
            Language::Russian => ru.into(),
        })
    }
    fn speaking(&self) -> bool {
        matches!(
            self.node,
            Node::Link | Node::Count | Node::Total | Node::Guided | Node::Final
        )
    }
    fn active(&self) -> bool {
        !matches!(self.node, Node::Idle | Node::Paused | Node::Finished)
    }
    fn target(&self) -> Option<u32> {
        if self.undo_add {
            return Some(self.fact().0);
        }
        match self.node {
            Node::Base => Some(self.fact().0),
            Node::Add | Node::Count | Node::Total | Node::Guided | Node::Final => Some(self.sum()),
            _ => None,
        }
    }
    fn phase(&self) -> PhaseKind {
        match self.node {
            Node::Idle => PhaseKind::Idle,
            Node::Link => PhaseKind::Meeting,
            Node::Base | Node::Add => PhaseKind::Performing,
            Node::Paused => PhaseKind::Paused,
            Node::Finished => PhaseKind::Finished,
            _ => PhaseKind::Asking,
        }
    }
    fn model(&self) -> String {
        (1..=self.sum())
            .map(|n| NumberWords::spoken(n, self.config.language))
            .collect::<Vec<_>>()
            .join(", ")
    }
    fn reminder_interval(&self) -> f64 {
        if self.config.nudge_interval.is_finite() {
            self.config.nudge_interval.clamp(1.0, 3600.0)
        } else {
            10.0
        }
    }
    fn prompt(&mut self) -> Vec<Effect> {
        self.sequence.reset();
        let mut es = vec![Effect::StopListening, Effect::CancelTimer(SETUP_REMINDER)];
        if self.undo_add {
            es.push(Effect::Say(crate::play::undo_added_toys(
                self.config.language,
            )));
            return es;
        }
        let (a, b) = self.fact();
        let sum = self.sum();
        let line = match self.node {
            Node::Link if self.link_retry => self.say("I couldn't hear clearly. Let's try once more: one, two, over.", "Не удалось расслышать. Скажи ещё раз спокойно: один, два, приём."),
            Node::Link => self.say("Hello! I'm Unnum. Let's check that I can hear you. Repeat: one, two, over.", "Привет! Я Уннум. Проверим, как я тебя слышу. Повтори: один, два, приём."),
            Node::Base => self.say(format!("Leave exactly {} on the table. Put the other toys aside and move your hands away.", toy_amount(a)), format!("Оставь на столе ровно {}. Остальные игрушки убери в сторону и убери руки.", toys(a))),
            Node::Add if b == 0 => self.say("Add zero toys: leave the toys as they are.", "Добавь ноль игрушек: оставь всё как есть."),
            Node::Add => self.say(format!("The starting group is ready. Add {} to make {} altogether, then move your hands away.", toy_amount(b), toy_amount(sum)), format!("Начальная группа готова. Добавь ещё {}, чтобы всего получилось {}. Убери руки.", toys(b), toy_group(sum))),
            Node::Count => self.say("Point to each toy one by one and count them all aloud, starting at one.", "Показывай на каждую игрушку по очереди и считай вслух. Начни с одного."),
            Node::Total => self.say("How many toys are there altogether?", "Сколько всего игрушек на столе?"),
            Node::Guided => self.say(format!("Let's count this same group together: {}. Point once to each toy. Your turn.", self.model()), format!("Посчитаем эту же группу вместе: {}. Покажи на каждую игрушку один раз. Теперь твоя очередь.", self.model())),
            Node::Final => self.say("Now count this same group again, all by yourself, starting at one.", "А теперь посчитай эту же группу самостоятельно, с самого начала. Начни с одного."),
            _ => return vec![],
        };
        es.push(crate::play::preface_retry(
            &mut self.retry_preface,
            self.config.language,
            line,
        ));
        if self.speaking() {
            es.push(Effect::listen(
                if self.node == Node::Total {
                    ListenMode::Voice
                } else {
                    ListenMode::Counting
                },
                self.config.answer_timeout,
            ));
        } else if self.node == Node::Base {
            es.push(Effect::StartTimer {
                token: SETUP_REMINDER,
                seconds: self.reminder_interval(),
            });
        }
        es
    }
    fn setup(&mut self) -> Vec<Effect> {
        self.node = Node::Base;
        self.undo_add = false;
        self.interrupted = None;
        self.last_setup_count = None;
        let mut es = self.prompt();
        es.extend(self.settle());
        es
    }
    fn settle(&mut self) -> Vec<Effect> {
        if self.table != self.target() {
            return vec![];
        }
        if self.undo_add {
            self.undo_add = false;
            let mut es = self.prompt();
            if self.fact().1 == 0 {
                es.extend(self.settle());
            }
            return es;
        }
        if self.node == Node::Base {
            self.node = Node::Add;
            let mut es = self.prompt();
            if self.fact().1 == 0 {
                es.extend(self.settle());
            }
            return es;
        }
        if self.node == Node::Add {
            self.node = self.interrupted.take().unwrap_or(Node::Count);
            return self.prompt();
        }
        vec![]
    }
    fn finish(&mut self, success: bool) -> Vec<Effect> {
        self.node = if success {
            Node::Finished
        } else {
            Node::Paused
        };
        let mut summary = SessionSummary {
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            final_level: self.index as u32,
            records: self.records.clone(),
            paused: !success,
            ..Default::default()
        };
        for key in [
            "unmath.lesson2.place",
            "unmath.lesson2.mastered",
            "unmath.lesson2.picnic.v2",
            OLD_PROGRESS,
        ] {
            summary.remember.insert(key.into(), String::new());
        }
        summary.remember.insert(
            PROGRESS.into(),
            if success {
                String::new()
            } else {
                self.index.to_string()
            },
        );
        let mut es = vec![Effect::StopListening, Effect::CancelTimer(SETUP_REMINDER)];
        if success {
            es.push(Effect::Celebrate);
        }
        es.push(if success {
            let name = self
                .config
                .child_name
                .as_deref()
                .filter(|n| !n.trim().is_empty())
                .map(|n| format!(", {n}"))
                .unwrap_or_default();
            self.say(
                format!("All eight examples are complete! Well done{name}. Goodbye!"),
                format!("Все восемь примеров решены! Отличная работа{name}. Пока!"),
            )
        } else {
            self.say(
                "Let's take a break. Your progress is saved; we can continue next time.",
                "Давай сделаем перерыв и отдохнём. Прогресс сохранён. В следующий раз продолжим.",
            )
        });
        es.push(Effect::SessionEnded(summary));
        es
    }
    fn budget_reached(&self) -> bool {
        self.records.len() as u32 >= self.config.max_tasks
    }
    fn answer(&mut self, n: Option<u32>, skipped: bool) -> Vec<Effect> {
        let numbers = self.sequence.take().unwrap_or_default();
        if self.node == Node::Link {
            if !skipped && valid_count(&numbers, 2, n) || self.link_retry {
                return self.setup();
            }
            self.link_retry = true;
            return self.prompt();
        }
        let sum = self.sum();
        let correct = !skipped
            && if self.node == Node::Total {
                n == Some(sum)
                    && (numbers.is_empty()
                        || numbers.iter().all(|v| *v == sum)
                        || valid_count(&numbers, sum, n))
            } else {
                valid_count(&numbers, sum, n)
            };
        let node = self.node;
        self.records.push(TaskRecord {
            task: format!(
                "{} + {} ({})",
                self.fact().0,
                self.fact().1,
                match node {
                    Node::Guided => "guided",
                    Node::Final => "independent recount",
                    Node::Total => "total",
                    _ => "independent count",
                }
            ),
            answers: if node == Node::Total && numbers.is_empty() {
                n.into_iter().collect()
            } else {
                numbers.clone()
            },
            expected: Some(sum),
            correct,
            skipped,
        });
        if correct && matches!(node, Node::Total | Node::Final) {
            self.index += 1;
            if self.index == FACTS.len() {
                return self.finish(true);
            }
            if self.budget_reached() {
                return self.finish(false);
            }
            let mut es = vec![self.say(
                "Correct! Let's try the next example.",
                "Верно! Переходим к следующему примеру.",
            )];
            es.extend(self.setup());
            return es;
        }
        if self.budget_reached() {
            return self.finish(false);
        }
        if correct {
            self.node = if node == Node::Count {
                Node::Total
            } else {
                Node::Final
            };
            return self.prompt();
        }
        if node == Node::Final {
            return self.finish(false);
        }
        self.retry_preface = !skipped;
        let mut es = if node == Node::Total {
            vec![self.say(
                "Let's check how many there are by counting each toy again.",
                "Проверим, сколько получилось: посчитаем каждую игрушку ещё раз.",
            )]
        } else {
            vec![count_feedback(&numbers, sum, self.config.language)]
        };
        if node == Node::Guided {
            es.push(self.say(
                format!(
                    "Watch and listen while I count this group: {}.",
                    self.model()
                ),
                format!(
                    "Посмотри и послушай, как я считаю эту группу: {}.",
                    self.model()
                ),
            ));
            self.node = Node::Final;
        } else {
            self.node = Node::Guided;
        }
        es.extend(self.prompt());
        es
    }
    fn observe(&mut self, t: TableObservation) -> Vec<Effect> {
        if !t.is_stable && t.count.is_some() && t.count == self.table {
            return vec![];
        }
        self.table = if t.is_stable { t.count } else { None };
        if !self.active() || self.node == Node::Link {
            return vec![];
        }
        if !self.undo_add && self.node != Node::Base && self.table.is_some_and(|n| n > self.sum()) {
            if self.speaking() {
                self.interrupted = Some(Node::Count);
            }
            self.node = Node::Add;
            self.undo_add = true;
            return self.prompt();
        }
        if self.speaking() && self.table != self.target() {
            self.interrupted = Some(self.node);
            self.node = Node::Add;
            self.sequence.reset();
            return vec![Effect::StopListening];
        }
        if self.node == Node::Base && self.table != self.target() {
            if let Some(n) = self.table {
                if self.last_setup_count != Some(n) {
                    self.last_setup_count = Some(n);
                    return self.setup_feedback(n);
                }
            }
        }
        if matches!(self.node, Node::Base | Node::Add) {
            self.settle()
        } else {
            vec![]
        }
    }
    fn setup_feedback(&self, n: u32) -> Vec<Effect> {
        let a = self.fact().0;
        vec![if n < a {
            self.say(
                format!(
                    "There are too few toys. Add toys until there are {} on the table.",
                    toy_amount(a)
                ),
                format!(
                    "Игрушек пока мало. Добавь игрушки, чтобы на столе {}.",
                    if a == 1 {
                        "была одна игрушка".to_owned()
                    } else {
                        format!("было {}", toy_group(a))
                    }
                ),
            )
        } else {
            self.say(format!("There are too many toys. Leave only {} on the table and put the others aside.", toy_amount(a)), format!("Игрушек слишком много. Оставь на столе только {}, а остальные убери в сторону.", toys(a)))
        }]
    }
}
impl Strategy for AddToFive {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        if matches!(&event, Event::Heard(n) if self.sequence.ignore_discarded_tail(*n)) {
            return vec![];
        }
        if matches!(&event, Event::Said(s) if wants_restart(s)) {
            return self.handle(Event::RepeatRequested);
        }
        match event {
            Event::Start if self.node == Node::Idle => {
                if self.config.max_tasks == 0 || self.config.total_objects < 5 {
                    return self.finish(false);
                }
                self.node = Node::Link;
                self.prompt()
            }
            Event::Stop if self.active() => self.finish(false),
            Event::Said(s) if self.active() && wants_break(&s) => self.finish(false),
            Event::Said(s) if self.speaking() && wants_help(&s) => {
                if self.node == Node::Link {
                    self.link_retry = true;
                } else {
                    self.node = Node::Guided;
                }
                self.prompt()
            }
            Event::Table(t) => self.observe(t),
            Event::Numbers(ns) if self.speaking() => {
                self.sequence.set(ns);
                vec![]
            }
            Event::Heard(n) if self.speaking() => self.answer(Some(n), false),
            Event::NothingHeard if self.speaking() => self.answer(None, true),
            Event::SkipRequested if self.active() => self.finish(false),
            Event::RepeatRequested if self.active() => {
                let mut es = self.prompt();
                es.extend(self.settle());
                es
            }
            Event::TimerFired(SETUP_REMINDER) if self.node == Node::Base => {
                let mut es = self
                    .table
                    .map(|n| self.setup_feedback(n))
                    .unwrap_or_default();
                es.push(Effect::StartTimer {
                    token: SETUP_REMINDER,
                    seconds: self.reminder_interval(),
                });
                es
            }
            _ => vec![],
        }
    }
    fn snapshot(&self) -> StrategySnapshot {
        StrategySnapshot {
            phase: self.phase(),
            level: self.index as u32,
            level_count: FACTS.len() as u32 + 1,
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            task: Some(if self.node == Node::Link {
                "Lesson 2: speech check".into()
            } else {
                format!(
                    "Lesson 2: {} + {} ({})",
                    self.fact().0,
                    self.fact().1,
                    match self.node {
                        Node::Guided => "guided",
                        Node::Final => "independent recount",
                        Node::Total => "total",
                        _ => "independent count",
                    }
                )
            }),
            target_count: self.target(),
            expected_answer: if self.node == Node::Link {
                Some(2)
            } else {
                self.speaking().then_some(self.sum())
            },
        }
    }
}
#[derive(Default)]
pub struct AddToFiveFactory;
impl StrategyFactory for AddToFiveFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo {
            id: ADDITION_ID.into(),
            title: "Lesson 2: add toys up to five".into(),
            description: "Join toy groups, count every toy, and say how many there are altogether."
                .into(),
            languages: vec![Language::English, Language::Russian],
            license: "LGPL-3.0-or-later".into(),
        }
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(AddToFive::new(config.clone()))
    }
}
inventory::submit! { LinkedStrategy(|| Arc::new(AddToFiveFactory)) }
