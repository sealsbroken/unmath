// SPDX-License-Identifier: LGPL-3.0-or-later
//! Lesson two follows the supplied bus graph («Lesson 2_3»): a warm-up, then two rounds on a
//! sheet of paper (the bus) with chips (the passengers). Round one is completion: one chip, make
//! three, say how many got on (2). Round two is addition: add two more, say how many ride (5).
use crate::answers::safe_config;
use crate::play::{wants_break, wants_help, wants_restart};
use std::sync::Arc;
use un_core::{language::Language, strategy::*};

pub const ADDITION_ID: &str = "unmath-add-to-five";
/// Keys of earlier versions of this lesson, cleared when a session ends.
const OLD_KEYS: [&str; 5] = [
    "unmath.lesson2.graph.v4.index",
    "unmath.lesson2.one_by_one.v3",
    "unmath.lesson2.picnic.v2",
    "unmath.lesson2.place",
    "unmath.lesson2.mastered",
];
/// Round one: one passenger, three in the end, so two got on.
const START: u32 = 1;
const GOAL: u32 = 3;
/// Round two: two more get on, five ride.
const MORE: u32 = 2;
const TOTAL: u32 = GOAL + MORE;
/// The silent pause once the chips are right, so hands can leave the table before it counts.
const HOLD: u64 = 201;
const HOLD_START: f64 = 3.0;
const HOLD_ADD: f64 = 4.0;
/// Repeats the instruction while the table stays wrong.
const REMINDER: u64 = 200;
/// How long the warm-up waits for the child to say something to the driver.
const WARMUP_SECONDS: f64 = 7.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Node {
    Idle,
    /// "Say something to the driver": any speech or 7 s of silence goes on.
    Warmup,
    /// One chip on the paper.
    Start,
    /// Make three, the child works out how many to add.
    Goal,
    /// How many got on? (2)
    Joined,
    /// Was one, now three: count the new chips with your finger.
    JoinedHelp,
    /// Two more get on: add two chips.
    Add,
    /// Too many: take the new chips off, back to three.
    Undo,
    /// How many ride now? (5)
    Total,
    /// We had three, two more came... that is...
    TotalHelp,
    Paused,
    Finished,
}

pub struct AddToFive {
    config: SessionConfig,
    node: Node,
    table: Option<u32>,
    /// The chips have been right since the hold timer started.
    holding: bool,
    /// The reminder timer is running.
    reminding: bool,
    records: Vec<TaskRecord>,
}
impl AddToFive {
    pub fn new(config: SessionConfig) -> Self {
        Self {
            config: safe_config(config),
            node: Node::Idle,
            table: None,
            holding: false,
            reminding: false,
            records: vec![],
        }
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
            Node::Warmup | Node::Joined | Node::JoinedHelp | Node::Total | Node::TotalHelp
        )
    }
    fn active(&self) -> bool {
        !matches!(self.node, Node::Idle | Node::Paused | Node::Finished)
    }
    fn target(&self) -> Option<u32> {
        match self.node {
            Node::Start => Some(START),
            Node::Goal | Node::Undo => Some(GOAL),
            Node::Add => Some(TOTAL),
            _ => None,
        }
    }
    fn expected(&self) -> Option<u32> {
        match self.node {
            Node::Joined | Node::JoinedHelp => Some(GOAL - START),
            Node::Total | Node::TotalHelp => Some(TOTAL),
            _ => None,
        }
    }
    fn round(&self) -> u32 {
        match self.node {
            Node::Idle
            | Node::Warmup
            | Node::Start
            | Node::Goal
            | Node::Joined
            | Node::JoinedHelp => 0,
            Node::Finished => 2,
            _ => 1,
        }
    }
    fn phase(&self) -> PhaseKind {
        match self.node {
            Node::Idle => PhaseKind::Idle,
            Node::Warmup => PhaseKind::Meeting,
            Node::Start | Node::Goal | Node::Add | Node::Undo => PhaseKind::Performing,
            Node::Paused => PhaseKind::Paused,
            Node::Finished => PhaseKind::Finished,
            _ => PhaseKind::Asking,
        }
    }
    fn reminder_interval(&self) -> f64 {
        if self.config.nudge_interval.is_finite() {
            self.config.nudge_interval.clamp(1.0, 3600.0)
        } else {
            10.0
        }
    }
    /// Cancels the running timers. Only running ones: the engine drops the next start of a
    /// timer whose cancel came while it was not running.
    fn cancel_timers(&mut self) -> Vec<Effect> {
        let mut es = vec![];
        if std::mem::take(&mut self.reminding) {
            es.push(Effect::CancelTimer(REMINDER));
        }
        if std::mem::take(&mut self.holding) {
            es.push(Effect::CancelTimer(HOLD));
        }
        es
    }
    /// The current node's line, then what it waits for.
    fn prompt(&mut self) -> Vec<Effect> {
        let mut es = vec![Effect::StopListening];
        es.extend(self.cancel_timers());
        es.push(match self.node {
            Node::Warmup => self.say(
                "Vroom! Our bus has started its engine. Say something to the driver, for example: let's go!",
                "Дрррм! Наш автобус завёл мотор. Скажи что-нибудь водителю, например: поехали!",
            ),
            Node::Start => self.say(
                "One passenger is sitting in the bus. Put one chip on the paper.",
                "В автобусе сидит один пассажир. Положи одну фишку.",
            ),
            Node::Goal => self.say(
                "At the bus stop the driver said: there must be three passengers in the bus altogether. Make it three.",
                "На остановке водитель сказал: всего в автобусе должно стать три пассажира. Сделай так, чтобы их стало три.",
            ),
            Node::Joined => self.say(
                "I can see three chips. You did it. Now tell me: how many passengers just got on the bus?",
                "Датчики видят три фишки. Ты выполнил задание. А теперь ответь: сколько именно пассажиров сейчас зашло в автобус?",
            ),
            Node::JoinedHelp => self.say(
                "Let's check. There was one... now there are three. How many did you add? Count the new chips with your finger.",
                "Давай проверим. Был один... Стало три. Сколько ты добавил? Считай пальчиком новые фишки.",
            ),
            Node::Add => self.say(
                "At the next stop two more passengers got on. Add two chips.",
                "На следующей остановке зашли ещё два пассажира. Добавь две фишки.",
            ),
            Node::Undo => self.say(
                "Oops, my window has fogged up. Take off the table the new chips you just put down.",
                "Ой, у меня запотело стекло. Убери со стола новые фишки, которые ты сейчас положил.",
            ),
            Node::Total => self.say(
                "The bus is off! How many passengers are riding in the bus now? Count them and say the whole number.",
                "Автобус поехал! Сколько теперь пассажиров едет в автобусе? Посчитай и назови общее число.",
            ),
            Node::TotalHelp => self.say(
                "The driver lost count. Let's do it together: we had three, and two more came... That makes...",
                "Водитель сбился со счёта. Давай вместе: у нас было три, и пришло ещё два... Это...",
            ),
            _ => return vec![],
        });
        match self.node {
            Node::Warmup => es.push(Effect::listen(ListenMode::Voice, WARMUP_SECONDS)),
            Node::Joined | Node::JoinedHelp => es.push(Effect::listen(
                ListenMode::Voice,
                self.config.answer_timeout,
            )),
            // Counting aloud up to five is welcome; the last number is the answer.
            Node::Total | Node::TotalHelp => es.push(Effect::listen(
                ListenMode::Counting,
                self.config.answer_timeout,
            )),
            _ => {
                self.reminding = true;
                es.push(Effect::StartTimer {
                    token: REMINDER,
                    seconds: self.reminder_interval(),
                });
                es.extend(self.check_table());
            }
        }
        es
    }
    fn go(&mut self, node: Node) -> Vec<Effect> {
        self.node = node;
        self.prompt()
    }
    /// Camera nodes: the right chips start the silent pause; anything else waits (and cancels it).
    fn check_table(&mut self) -> Vec<Effect> {
        let Some(target) = self.target() else {
            return vec![];
        };
        if self.node == Node::Add && self.table.is_some_and(|n| n > TOTAL) {
            return self.go(Node::Undo);
        }
        if self.table == Some(target) {
            if self.holding {
                return vec![];
            }
            self.holding = true;
            let seconds = match self.node {
                Node::Start => HOLD_START,
                Node::Undo => 0.0,
                _ => HOLD_ADD,
            };
            if seconds == 0.0 {
                return self.held();
            }
            return vec![Effect::StartTimer {
                token: HOLD,
                seconds,
            }];
        }
        if self.holding {
            self.holding = false;
            return vec![Effect::CancelTimer(HOLD)];
        }
        vec![]
    }
    /// The pause is over and the chips are still right.
    fn held(&mut self) -> Vec<Effect> {
        self.holding = false;
        match self.node {
            Node::Start => self.go(Node::Goal),
            Node::Goal => self.go(Node::Joined),
            Node::Undo => self.go(Node::Add),
            Node::Add => self.go(Node::Total),
            _ => vec![],
        }
    }
    fn record(&mut self, n: Option<u32>, skipped: bool) -> bool {
        let expected = self.expected();
        let correct = !skipped && n.is_some() && n == expected;
        self.records.push(TaskRecord {
            task: match self.node {
                Node::Joined => "1 + ? = 3 (how many got on)",
                Node::JoinedHelp => "1 + ? = 3 (guided)",
                Node::Total => "3 + 2 (how many ride)",
                _ => "3 + 2 (guided)",
            }
            .into(),
            answers: n.into_iter().collect(),
            expected,
            correct,
            skipped,
        });
        correct
    }
    fn answer(&mut self, n: Option<u32>, skipped: bool) -> Vec<Effect> {
        if self.node == Node::Warmup {
            let mut es = vec![
                Effect::StopListening,
                self.say(
                    "Great, off we go! Our bus is the sheet of paper on the table.",
                    "Отлично, едем! Наш автобус — это лист бумаги на столе.",
                ),
            ];
            es.extend(self.go(Node::Start));
            return es;
        }
        let correct = self.record(n, skipped);
        match self.node {
            Node::Joined if !correct => self.go(Node::JoinedHelp),
            Node::Joined | Node::JoinedHelp => {
                let line = if correct {
                    self.say(
                        "Exactly, two! The radar has recorded what you did. Let's move on.",
                        "Точно, два! Радар зафиксировал твоё действие. Двигаемся дальше.",
                    )
                } else {
                    self.say(
                        "Two passengers got on: one and two. Let's move on.",
                        "Зашло два пассажира: раз и два. Двигаемся дальше.",
                    )
                };
                let mut es = vec![
                    Effect::StopListening,
                    line,
                    self.say(
                        "You already know that three passengers are riding in the bus now. They are in their seats.",
                        "Ты уже знаешь, что сейчас в автобусе едут три пассажира. Они сидят на местах.",
                    ),
                ];
                es.extend(self.go(Node::Add));
                es
            }
            Node::Total if !correct => self.go(Node::TotalHelp),
            _ => self.finish(true),
        }
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
            final_level: self.round(),
            records: self.records.clone(),
            paused: !success,
            ..Default::default()
        };
        for key in OLD_KEYS {
            summary.remember.insert(key.into(), String::new());
        }
        let mut es = vec![Effect::StopListening];
        es.extend(self.cancel_timers());
        if success {
            es.push(Effect::Celebrate);
            es.push(self.say(
                "Five! Everyone is here. You did great today: you worked out how many passengers got on, and you counted them all at the end of the ride. Goodbye!",
                "Пять! Все на месте. Сегодня ты отлично справился: сам угадал, сколько пассажиров зашло, и правильно сосчитал их в конце поездки. Пока!",
            ));
        } else {
            es.push(self.say(
                "Let's take a break. We can ride the bus again next time.",
                "Давай сделаем перерыв и отдохнём. В следующий раз снова покатаемся на автобусе.",
            ));
        }
        es.push(Effect::SessionEnded(summary));
        es
    }
    fn observe(&mut self, t: TableObservation) -> Vec<Effect> {
        self.table = if t.is_stable { t.count } else { None };
        if self.target().is_some() {
            self.check_table()
        } else {
            vec![]
        }
    }
}
impl Strategy for AddToFive {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        if matches!(&event, Event::Said(s) if wants_restart(s)) {
            return self.handle(Event::RepeatRequested);
        }
        match event {
            Event::Start if self.node == Node::Idle => {
                if self.config.max_tasks == 0 || self.config.total_objects < TOTAL {
                    return self.finish(false);
                }
                self.go(Node::Warmup)
            }
            Event::Stop if self.active() => self.finish(false),
            Event::Said(s) if self.active() && wants_break(&s) => self.finish(false),
            // Anything said to the driver starts the ride.
            Event::Said(_) if self.node == Node::Warmup => self.answer(None, false),
            Event::Said(s) if wants_help(&s) && self.node == Node::Joined => {
                self.go(Node::JoinedHelp)
            }
            Event::Said(s) if wants_help(&s) && self.node == Node::Total => {
                self.go(Node::TotalHelp)
            }
            Event::Table(t) => self.observe(t),
            Event::Heard(n) if self.speaking() => self.answer(Some(n), false),
            Event::NothingHeard if self.speaking() => self.answer(None, true),
            Event::SkipRequested if self.active() => self.finish(false),
            Event::RepeatRequested if self.active() => self.prompt(),
            Event::TimerFired(HOLD) if self.holding && self.table == self.target() => self.held(),
            Event::TimerFired(REMINDER) if self.reminding && self.target().is_some() => {
                self.reminding = false;
                self.prompt()
            }
            _ => vec![],
        }
    }
    fn snapshot(&self) -> StrategySnapshot {
        StrategySnapshot {
            phase: self.phase(),
            level: self.round(),
            level_count: 2,
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            task: Some(
                match self.node {
                    Node::Warmup => "Lesson 2: the bus starts",
                    Node::Start => "Lesson 2: one passenger",
                    Node::Goal => "Lesson 2: make three",
                    Node::Joined | Node::JoinedHelp => "Lesson 2: 1 + ? = 3",
                    Node::Add | Node::Undo => "Lesson 2: two more get on",
                    _ => "Lesson 2: 3 + 2",
                }
                .into(),
            ),
            target_count: self.target(),
            expected_answer: self.expected(),
        }
    }
}
#[derive(Default)]
pub struct AddToFiveFactory;
impl StrategyFactory for AddToFiveFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo {
            id: ADDITION_ID.into(),
            title: "Lesson 2: the bus (adding to five)".into(),
            description: "Make three passengers from one and say how many got on, then add two and say how many ride."
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
