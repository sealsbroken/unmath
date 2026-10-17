// SPDX-License-Identifier: LGPL-3.0-or-later
//! Addition with nonnegative operands and a total no greater than five.
use crate::play::{invitation, wants_break, wants_help, Place};
use std::sync::Arc;
use un_core::{language::Language, strategy::*};

pub const ADDITION_ID: &str = "unmath-add-to-five";
const PROGRESS: &str = "unmath.lesson2.picnic.v2";
// Seven picnic missions, including two different ways to make five and adding zero.
const FACTS: [(u32, u32); 7] = [(1, 1), (1, 2), (2, 2), (2, 3), (3, 1), (1, 4), (3, 0)];
const ALL: u32 = (1 << FACTS.len()) - 1;
#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Idle,
    Meeting,
    Base,
    Add,
    Count,
    Check,
    Paused,
    Finished,
}

/// A picnic adventure: combine toy supplies, then solve seven short story missions.
/// Toy-assisted answers are practice; only unprompted Check answers earn mastery.
pub struct AddToFive {
    config: SessionConfig,
    place: Place,
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
        let place = config
            .extra
            .get("unmath.lesson2.place")
            .and_then(|s| Place::from_words(s))
            .unwrap_or_default();
        Self {
            place,
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
            Node::Meeting => PhaseKind::Meeting,
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
        let friends = self.place.friends(self.config.language);
        let line = match self.node {
            Node::Base => self.say(format!("Let's pack a picnic for our friends! Pretend each toy is a snack. Put {a} toy snacks on the table, then move your hands away. A grown-up can help."), format!("Собираем пикник для друзей! Пусть игрушки будут угощениями. Положи на стол {a} игрушек и убери руки. Взрослый может помочь.")),
            Node::Add => self.say(if b == 0 { "Our friend brought an empty basket! Add zero: leave the toy snacks as they are.".into() } else { format!("Our friend brings {b} more snacks! Can you add {b} toys to our {a}, then move your hands away?") }, if b == 0 { "Друг принёс пустую корзинку! Прибавь ноль: оставь игрушечные угощения как есть.".into() } else { format!("Друг принёс ещё {b} угощений! Добавь {b} игрушек к нашим {a} и убери руки.") }),
            Node::Count if sum > 0 => self.say(format!("Our picnic basket has {a} snacks and {b} more. Can you count all the toy snacks aloud?"), format!("В корзинке {a} угощений и ещё {b}. Посчитай все игрушечные угощения вслух.")),
            Node::Count => self.say("There are no toys. How many is that?".into(), "Игрушек нет. Сколько это?".into()),
            Node::Check => self.say(format!("Picnic mission {} of 7! The {friends} have {a} snacks and bring {b} more. How many altogether? You can use your fingers.", self.mastered.count_ones() + 1), format!("Задание для пикника {} из 7! У {friends} есть {a} угощений, они принесли ещё {b}. Сколько всего? Можно считать на пальцах.", self.mastered.count_ones() + 1)),
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
            "unmath.lesson2.place".into(),
            if success {
                String::new()
            } else {
                self.place.saved().into()
            },
        );
        summary
            .remember
            .insert("unmath.lesson2.mastered".into(), String::new());
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
                "The picnic is ready! You joined our snacks and found how many altogether. Time for a pretend feast!".into(),
                "Пикник готов! Ты соединил угощения и узнал, сколько всего. Пора играть в праздничный обед!".into(),
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
            if !guided {
                effects.push(Effect::Celebrate);
                let (en, ru) = match self.mastered.count_ones() {
                    1 => (
                        "Spread our pretend picnic blanket!",
                        "Расстели воображаемое одеяло для пикника!",
                    ),
                    2 => (
                        "Pass a pretend basket to a friend!",
                        "Передай другу воображаемую корзинку!",
                    ),
                    3 => (
                        "Pretend to pour a drink. Glug glug!",
                        "Налей воображаемый сок. Буль-буль!",
                    ),
                    4 => (
                        "Smell our pretend soup. Mmm!",
                        "Понюхай воображаемый суп. М-м-м!",
                    ),
                    5 => (
                        "Offer a pretend snack to your favorite toy!",
                        "Угости любимую игрушку воображаемым угощением!",
                    ),
                    _ => (
                        "Give our picnic chef a high five!",
                        "Дай пять нашему повару!",
                    ),
                };
                effects.push(self.say(en.into(), ru.into()));
                effects.push(self.say(
                    format!(
                        "Basket packed! {} picnic missions complete.",
                        self.mastered.count_ones()
                    ),
                    format!(
                        "Корзинка собрана! Выполнено заданий: {}.",
                        self.mastered.count_ones()
                    ),
                ));
            }
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
                    self.node = Node::Meeting;
                    invitation(&self.config)
                } else {
                    self.next_check()
                }
            }
            Event::Heard(n) if self.node == Node::Meeting => {
                if let Some(place) = Place::from_number(n) {
                    self.place = place;
                    self.misses = 0;
                    self.practice()
                } else {
                    invitation(&self.config)
                }
            }
            Event::Said(words)
                if wants_break(&words)
                    && matches!(
                        self.node,
                        Node::Meeting | Node::Base | Node::Add | Node::Count | Node::Check
                    ) =>
            {
                self.finish(false)
            }
            Event::Said(words)
                if wants_help(&words) && matches!(self.node, Node::Count | Node::Check) =>
            {
                self.practice()
            }
            Event::Said(words) if self.node == Node::Meeting => {
                if let Some(place) = Place::from_words(&words) {
                    self.place = place;
                    self.misses = 0;
                    self.practice()
                } else {
                    invitation(&self.config)
                }
            }
            Event::RepeatRequested if self.node == Node::Meeting => invitation(&self.config),
            Event::NothingHeard if self.node == Node::Meeting => {
                self.misses += 1;
                if self.misses >= 3 {
                    self.finish(false)
                } else {
                    invitation(&self.config)
                }
            }
            Event::SkipRequested if self.node == Node::Meeting => self.practice(),
            Event::Stop
                if matches!(
                    self.node,
                    Node::Meeting | Node::Base | Node::Add | Node::Count | Node::Check
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
            expected_answer: matches!(self.node, Node::Count | Node::Check)
                .then_some(self.fact.0 + self.fact.1),
        }
    }
}
#[derive(Default)]
pub struct AddToFiveFactory;
impl StrategyFactory for AddToFiveFactory {
    fn info(&self) -> StrategyInfo {
        StrategyInfo { id: ADDITION_ID.into(), title: "Lesson 2: picnic baskets".into(),
            description: "Pack a pretend picnic, join toy snacks, and solve seven story missions with totals up to five.".into(),
            languages: vec![Language::English, Language::Russian], license: "LGPL-3.0-or-later".into() }
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(AddToFive::new(config.clone()))
    }
}
inventory::submit! { LinkedStrategy(|| Arc::new(AddToFiveFactory)) }
