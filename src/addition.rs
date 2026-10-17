//! Addition with nonnegative operands and a total no greater than five.
use std::sync::Arc;
use un_core::{language::Language, strategy::*};

pub const ADDITION_ID: &str = "unmath-add-to-five";
const PROGRESS: &str = "unmath.lesson2.mastered";
// Increasing totals, including adding zero on either side.
const FACTS: [(u32, u32); 21] = [
    (0, 0),
    (0, 1),
    (1, 0),
    (1, 1),
    (0, 2),
    (2, 0),
    (1, 2),
    (2, 1),
    (0, 3),
    (3, 0),
    (2, 2),
    (1, 3),
    (3, 1),
    (0, 4),
    (4, 0),
    (2, 3),
    (3, 2),
    (1, 4),
    (4, 1),
    (0, 5),
    (5, 0),
];
const ALL: u32 = (1 << FACTS.len()) - 1;
#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Idle,
    Base,
    Add,
    Count,
    Check,
    Paused,
    Finished,
}

/// Teach combining sets, then check every ordered addition fact with total <= 5.
/// Toy-assisted answers are practice; only unprompted Check answers earn mastery.
pub struct AddToFive {
    config: SessionConfig,
    node: Node,
    index: usize,
    fact: (u32, u32),
    table: Option<u32>,
    sequence: Option<Vec<u32>>,
    mastered: u32,
    misses: u32,
    records: Vec<TaskRecord>,
}
impl AddToFive {
    pub fn new(config: SessionConfig) -> Self {
        let mastered = config
            .extra
            .get(PROGRESS)
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0)
            & ALL;
        Self {
            config,
            node: Node::Idle,
            index: 0,
            fact: (1, 1),
            table: None,
            sequence: None,
            mastered,
            misses: 0,
            records: vec![],
        }
    }
    fn say(&self, en: String, ru: String) -> Effect {
        Effect::Say(match self.config.language {
            Language::English => en,
            Language::Russian => ru,
        })
    }
    fn target(&self) -> Option<u32> {
        match self.node {
            Node::Base => Some(self.fact.0),
            Node::Add | Node::Count => Some(self.fact.0 + self.fact.1),
            _ => None,
        }
    }
    fn phase(&self) -> PhaseKind {
        match self.node {
            Node::Idle => PhaseKind::Idle,
            Node::Base | Node::Add => PhaseKind::Performing,
            Node::Count | Node::Check => PhaseKind::Asking,
            Node::Paused => PhaseKind::Paused,
            Node::Finished => PhaseKind::Finished,
        }
    }
    fn prompt(&mut self) -> Vec<Effect> {
        self.sequence = None;
        let (a, b) = self.fact;
        let sum = a + b;
        let line = match self.node {
            Node::Base => self.say(format!("Let's add with toys. Put {a} toys on the table. A grown-up can help. Move your hands away when ready."), format!("Давай складывать с игрушками. Положи на стол {a} игрушек. Взрослый может помочь. Когда закончишь, убери руки.")),
            Node::Add => self.say(if b == 0 { "Add zero: leave the toys as they are.".into() } else { format!("There are {a} toys. Add {b} more toys, then move your hands away.") }, if b == 0 { "Прибавь ноль: оставь игрушки как есть.".into() } else { format!("На столе {a} игрушек. Добавь ещё {b} и убери руки.") }),
            Node::Count if sum > 0 => self.say(format!("We joined {a} and {b}. Can you count all the toys aloud?"), format!("Мы соединили {a} и {b}. Посчитай все игрушки вслух.")),
            Node::Count => self.say("There are no toys. How many is that?".into(), "Игрушек нет. Сколько это?".into()),
            Node::Check => self.say(format!("What is {a} plus {b}? You can use your fingers."), format!("Сколько будет {a} плюс {b}? Можно считать на пальцах.")),
            _ => return vec![],
        };
        let mut effects = vec![Effect::StopListening, line];
        if matches!(self.node, Node::Count | Node::Check) {
            effects.push(Effect::listen(
                if self.node == Node::Count && sum > 0 {
                    ListenMode::Counting
                } else {
                    ListenMode::Voice
                },
                self.config.answer_timeout,
            ));
        }
        effects
    }
    fn settle(&mut self) -> Vec<Effect> {
        let mut effects = vec![];
        if self.table != self.target() {
            return effects;
        }
        if self.node == Node::Base {
            self.node = Node::Add;
            effects.extend(self.prompt());
            // Adding zero is a real operation but needs no camera change.
            if self.fact.1 != 0 {
                return effects;
            }
        }
        if self.node == Node::Add {
            self.node = Node::Count;
            effects.extend(self.prompt());
        }
        effects
    }
    fn practice(&mut self) -> Vec<Effect> {
        self.node = Node::Base;
        let mut effects = self.prompt();
        effects.extend(self.settle());
        effects
    }
    fn next_check(&mut self) -> Vec<Effect> {
        if self.mastered == ALL {
            return self.finish(true);
        }
        while self.mastered & (1 << self.index) != 0 {
            self.index = (self.index + 1) % FACTS.len();
        }
        self.fact = FACTS[self.index];
        self.node = Node::Check;
        self.prompt()
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
            final_level: self.mastered.count_ones(),
            records: self.records.clone(),
            paused: !success,
            ..Default::default()
        };
        summary.remember.insert(
            PROGRESS.into(),
            if success {
                String::new()
            } else {
                self.mastered.to_string()
            },
        );
        let mut effects = vec![Effect::StopListening];
        if success {
            effects.push(Effect::Celebrate);
        }
        effects.push(if success {
            self.say(
                "You can add numbers with totals up to five!".into(),
                "Ты умеешь складывать числа с суммой до пяти!".into(),
            )
        } else {
            self.say(
                "Let's take a break. We can add again next time.".into(),
                "Давай отдохнём. В следующий раз продолжим складывать.".into(),
            )
        });
        effects.push(Effect::SessionEnded(summary));
        effects
    }
    fn answer(&mut self, answer: Option<u32>, skipped: bool) -> Vec<Effect> {
        let sum = self.fact.0 + self.fact.1;
        let sequence = self.sequence.take().unwrap_or_default();
        let guided = self.node == Node::Count;
        let correct = !skipped
            && if guided && sum > 0 {
                sequence == (1..=sum).collect::<Vec<_>>()
            } else {
                answer == Some(sum)
            };
        self.records.push(TaskRecord {
            task: format!(
                "{} + {} ({})",
                self.fact.0,
                self.fact.1,
                if guided { "guided" } else { "independent" }
            ),
            answers: if guided && sum > 0 {
                sequence
            } else {
                answer.into_iter().collect()
            },
            expected: Some(sum),
            correct,
            skipped,
        });
        if correct {
            self.misses = 0;
            if !guided {
                self.mastered |= 1 << self.index;
                self.index = (self.index + 1) % FACTS.len();
            }
        } else {
            self.misses += 1;
        }
        if self.mastered == ALL {
            return self.finish(true);
        }
        if self.records.len() as u32 >= self.config.max_tasks.max(1) || self.misses >= 3 {
            return self.finish(false);
        }
        if correct {
            let mut effects = vec![];
            if guided {
                effects.push(self.say(
                    format!(
                        "{0} plus {1} makes {sum}. Adding joins the toys together.",
                        self.fact.0, self.fact.1
                    ),
                    format!(
                        "{0} плюс {1} равно {sum}. При сложении мы соединяем игрушки.",
                        self.fact.0, self.fact.1
                    ),
                ));
                // Ask a different fact before revisiting a failed fact when possible.
                self.index = (self.index + 1) % FACTS.len();
            }
            effects.extend(self.next_check());
            effects
        } else {
            let mut effects = vec![];
            if guided {
                effects.push(Effect::HighlightCount(sum));
                effects.push(self.say(
                    format!("There are {sum} altogether. Let's try joining the toys again."),
                    format!("Всего {sum}. Попробуем ещё раз соединить игрушки."),
                ));
            }
            effects.extend(self.practice());
            effects
        }
    }
}
impl Strategy for AddToFive {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        match event {
            Event::Start if self.node == Node::Idle => {
                if self.mastered == 0 {
                    self.practice()
                } else {
                    self.next_check()
                }
            }
            Event::Stop
                if matches!(
                    self.node,
                    Node::Base | Node::Add | Node::Count | Node::Check
                ) =>
            {
                self.finish(false)
            }
            Event::Table(t) => {
                self.table = if t.is_stable { t.count } else { None };
                if self.node == Node::Count && self.table != self.target() {
                    self.sequence = None;
                    return self.practice();
                }
                if matches!(self.node, Node::Base | Node::Add) {
                    self.settle()
                } else {
                    vec![]
                }
            }
            Event::Numbers(ns) if matches!(self.node, Node::Count | Node::Check) => {
                self.sequence = Some(ns);
                vec![]
            }
            Event::Heard(n) if matches!(self.node, Node::Count | Node::Check) => {
                self.answer(Some(n), false)
            }
            Event::NothingHeard if matches!(self.node, Node::Count | Node::Check) => {
                self.misses += 1;
                if self.misses >= 3 {
                    self.finish(false)
                } else {
                    self.prompt()
                }
            }
            Event::RepeatRequested
                if matches!(
                    self.node,
                    Node::Base | Node::Add | Node::Count | Node::Check
                ) =>
            {
                let mut effects = self.prompt();
                if matches!(self.node, Node::Base | Node::Add) {
                    effects.extend(self.settle());
                }
                effects
            }
            Event::SkipRequested if matches!(self.node, Node::Count | Node::Check) => {
                self.answer(None, true)
            }
            Event::SkipRequested if matches!(self.node, Node::Base | Node::Add) => {
                self.finish(false)
            }
            _ => vec![],
        }
    }
    fn snapshot(&self) -> StrategySnapshot {
        StrategySnapshot {
            phase: self.phase(),
            level: self.mastered.count_ones(),
            level_count: FACTS.len() as u32 + 1,
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            task: Some(format!(
                "Lesson 2: {} + {} ({})",
                self.fact.0,
                self.fact.1,
                if self.node == Node::Check {
                    "independent"
                } else {
                    "guided"
                }
            )),
            target_count: self.target(),
            expected_answer: Some(self.fact.0 + self.fact.1),
        }
    }
}
#[derive(Default)]
pub struct AddToFiveFactory;
impl StrategyFactory for AddToFiveFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo { id: ADDITION_ID.into(), title: "Lesson 2: add up to five".into(),
            description: "Join toy sets, then independently solve every addition fact with a total up to five.".into(),
            languages: vec![Language::English, Language::Russian], license: "UNLICENSED".into() }
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(AddToFive::new(config.clone()))
    }
}
inventory::submit! { LinkedStrategy(|| Arc::new(AddToFiveFactory)) }
