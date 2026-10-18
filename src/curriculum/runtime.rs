// SPDX-License-Identifier: LGPL-3.0-or-later
use super::*;
use crate::answers::{count_feedback, safe_config, saved_mask, valid_count, CountBuffer};
use crate::play::{invitation, normalized, wants_break, wants_help, wants_restart, Place};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Idle,
    Choose,
    Check,
    Prepare,
    Guide,
    Paused,
    Finished,
}

/// Reusable event reducer for eight distinct question/action curricula.
/// A guided answer never completes a mission. Only an independent check can do so.
pub struct Adventure {
    lesson: Lesson,
    config: SessionConfig,
    missions: Vec<Mission>,
    node: Node,
    index: usize,
    stage: usize,
    mastered: u32,
    place: Place,
    table: Option<u32>,
    sequence: CountBuffer,
    misses: u32,
    records: Vec<TaskRecord>,
}
impl Adventure {
    pub fn new(lesson: Lesson, config: SessionConfig) -> Self {
        let config = safe_config(config);
        let missions = lesson.missions();
        let key = lesson.key();
        let mastered = saved_mask(config.extra.get(&key), (1 << missions.len()) - 1);
        let index = config
            .extra
            .get(&format!("{key}.index"))
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|i| *i < missions.len())
            .unwrap_or(0);
        let place = config
            .extra
            .get(&format!("{key}.place"))
            .and_then(|s| Place::from_words(s))
            .unwrap_or_default();
        Self {
            lesson,
            config,
            missions,
            node: Node::Idle,
            index,
            stage: 0,
            mastered,
            place,
            table: None,
            sequence: CountBuffer::default(),
            misses: 0,
            records: vec![],
        }
    }
    fn mission(&self) -> &Mission {
        &self.missions[self.index]
    }
    fn phase(&self) -> PhaseKind {
        match self.node {
            Node::Idle => PhaseKind::Idle,
            Node::Choose => PhaseKind::Meeting,
            Node::Check | Node::Guide => PhaseKind::Asking,
            Node::Prepare => PhaseKind::Performing,
            Node::Paused => PhaseKind::Paused,
            Node::Finished => PhaseKind::Finished,
        }
    }
    fn say(&self, words: &Words) -> Effect {
        Effect::Say(words.in_language(self.config.language))
    }
    fn target(&self) -> Option<u32> {
        matches!(self.node, Node::Prepare | Node::Guide)
            .then(|| self.mission().stages[self.stage].count)
    }
    fn expected(&self) -> Option<u32> {
        match self.node {
            Node::Choose => None,
            Node::Check => Some(self.mission().answer.expected()),
            Node::Guide => self.target(),
            _ => None,
        }
    }
    fn check(&mut self) -> Vec<Effect> {
        if self.mastered.count_ones() as usize == self.missions.len() {
            return self.finish(true);
        }
        while self.mastered & (1 << self.index) != 0 {
            self.index = (self.index + 1) % self.missions.len();
        }
        self.node = Node::Check;
        self.sequence.reset();
        let destination = match self.place {
            Place::Forest => Words::new("Our forest friends need you!", "Лесные друзья ждут тебя!"),
            Place::Moon => Words::new("Our moon friends need you!", "Лунные друзья ждут тебя!"),
        };
        vec![
            Effect::StopListening,
            self.say(&destination),
            self.say(&self.mission().question),
            Effect::listen(
                if matches!(self.mission().answer, Answer::Sequence(_)) {
                    ListenMode::Counting
                } else {
                    ListenMode::Voice
                },
                self.config.answer_timeout,
            ),
        ]
    }
    fn guide(&mut self) -> Vec<Effect> {
        self.stage = 0;
        self.prepare()
    }
    fn prepare(&mut self) -> Vec<Effect> {
        self.node = Node::Prepare;
        self.sequence.reset();
        let mut effects = vec![
            Effect::StopListening,
            self.say(&self.mission().stages[self.stage].action),
        ];
        if self.table == self.target() {
            effects.extend(self.guide_question());
        }
        effects
    }
    fn guide_question(&mut self) -> Vec<Effect> {
        self.node = Node::Guide;
        self.sequence.reset();
        let n = self.target().expect("a guide always has a target");
        vec![
            self.say(&if n == 0 {
                Words::new(
                    "Nothing is here now. How many is that?",
                    "Теперь здесь ничего нет. Сколько это?",
                )
            } else {
                Words::new(
                    "Point to the toys one by one. Say one number for each toy, starting at one, until every toy has been counted. We can take our time!",
                    "Показывай на каждую игрушку по очереди и считай. Начни с одного и посчитай все игрушки. Мы не спешим!",
                )
            }),
            Effect::listen(
                if n == 0 {
                    ListenMode::Voice
                } else {
                    ListenMode::Counting
                },
                self.config.answer_timeout,
            ),
        ]
    }
    fn answer(&mut self, number: Option<u32>, skipped: bool) -> Vec<Effect> {
        let sequence = self.sequence.take().unwrap_or_default();
        let guided = self.node == Node::Guide;
        let expected = self.expected().expect("answers only in questions");
        let needs_sequence = if guided {
            expected > 0
        } else {
            matches!(self.mission().answer, Answer::Sequence(_))
        };
        let correct = !skipped
            && if needs_sequence {
                valid_count(&sequence, expected, number)
            } else {
                number == Some(expected)
            };
        let feedback = (!correct && !skipped && needs_sequence)
            .then(|| count_feedback(&sequence, expected, self.config.language));
        self.records.push(TaskRecord {
            task: format!(
                "{}: {}{}",
                self.mission().name,
                if guided { "guided" } else { "independent" },
                if guided {
                    format!(" stage {}", self.stage + 1)
                } else {
                    String::new()
                }
            ),
            answers: if needs_sequence {
                sequence
            } else {
                number.into_iter().collect()
            },
            expected: Some(expected),
            correct,
            skipped,
        });
        if correct {
            self.misses = 0;
            if !guided {
                self.mastered |= 1 << self.index;
            }
        } else {
            self.misses += 1;
        }
        if self.mastered.count_ones() as usize == self.missions.len() {
            return self.finish(true);
        }
        if self.records.len() as u32 >= self.config.max_tasks.max(1) || self.misses >= 3 {
            return self.finish(false);
        }
        if !correct {
            let mut effects = vec![Effect::StopListening];
            effects.extend(feedback);
            if guided {
                if expected > 0 {
                    effects.push(Effect::HighlightCount(expected));
                }
                let count = if expected > 0 {
                    (1..=expected)
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    "0".into()
                };
                effects.push(self.say(&Words::new(
                    format!("Let's count together: {count}. Try the toys again."),
                    format!("Посчитаем вместе: {count}. Попробуем с игрушками ещё раз."),
                )));
            }
            effects.extend(self.guide());
            return effects;
        }
        if guided {
            self.stage += 1;
            if self.stage < self.mission().stages.len() {
                return self.prepare();
            }
            let mut effects = vec![Effect::StopListening, self.say(&self.mission().explanation)];
            // Leave this mission unfinished; another unfinished mission comes first if possible.
            self.index = (self.index + 1) % self.missions.len();
            effects.extend(self.check());
            effects
        } else {
            let mut effects = vec![
                Effect::StopListening,
                Effect::Celebrate,
                self.say(&self.mission().reward),
            ];
            self.index = (self.index + 1) % self.missions.len();
            effects.extend(self.check());
            effects
        }
    }
    fn finish(&mut self, success: bool) -> Vec<Effect> {
        self.node = if success {
            Node::Finished
        } else {
            Node::Paused
        };
        self.sequence.reset();
        let key = self.lesson.key();
        let mut summary = SessionSummary {
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            final_level: self.mastered.count_ones(),
            records: self.records.clone(),
            paused: !success,
            ..Default::default()
        };
        for (k, v) in [
            (key.clone(), self.mastered.to_string()),
            (format!("{key}.index"), self.index.to_string()),
            (format!("{key}.place"), self.place.saved().into()),
        ] {
            summary
                .remember
                .insert(k, if success { String::new() } else { v });
        }
        let mut effects = vec![Effect::StopListening];
        if success {
            effects.push(Effect::Celebrate);
        }
        effects.push(self.say(&if success {Words::new("Adventure complete! Give your toy friends a cheer. You solved every mission!", "Приключение завершено! Порадуйся вместе с игрушечными друзьями. Все задания выполнены!")} else {Words::new("Let's rest. Our friends will keep this adventure ready for next time.", "Давай отдохнём. Продолжим приключение в следующий раз.")}));
        effects.push(Effect::SessionEnded(summary));
        effects
    }
    fn choose(&mut self, place: Option<Place>) -> Vec<Effect> {
        if let Some(place) = place {
            self.place = place;
            self.misses = 0;
            let mut effects = vec![self.say(&self.lesson.welcome())];
            effects.extend(self.check());
            effects
        } else {
            self.misses += 1;
            if self.misses >= 3 {
                self.finish(false)
            } else {
                invitation(&self.config)
            }
        }
    }
    fn active(&self) -> bool {
        matches!(
            self.node,
            Node::Choose | Node::Check | Node::Prepare | Node::Guide
        )
    }
}
impl Strategy for Adventure {
    fn handle(&mut self, event: Event) -> Vec<Effect> {
        if matches!(&event, Event::Heard(n) if self.sequence.ignore_discarded_tail(*n)) {
            return vec![];
        }
        if matches!(&event, Event::Said(words) if wants_restart(words)) {
            return self.handle(Event::RepeatRequested);
        }
        match event {
            Event::Start if self.node == Node::Idle => {
                if self.config.max_tasks == 0 {
                    return self.finish(false);
                }
                if self.config.total_objects < self.lesson.required_toys() {
                    let mut effects=vec![self.say(&Words::new(format!("Ask a grown-up to find {} recognizable toys for this adventure. We can play it when they're ready.",self.lesson.required_toys()),format!("Попроси взрослого подготовить {} для игры. Продолжим, когда всё будет готово.",crate::russian::toys(self.lesson.required_toys()))))];
                    effects.extend(self.finish(false));
                    return effects;
                }
                if self
                    .config
                    .extra
                    .get(&format!("{}.place", self.lesson.key()))
                    .and_then(|s| Place::from_words(s))
                    .is_some()
                {
                    let mut effects = vec![self.say(&self.lesson.welcome())];
                    effects.extend(self.check());
                    effects
                } else {
                    self.node = Node::Choose;
                    invitation(&self.config)
                }
            }
            Event::Stop if self.active() => self.finish(false),
            Event::Said(s) if self.active() && wants_break(&s) => self.finish(false),
            Event::Heard(n) if self.node == Node::Choose => self.choose(Place::from_number(n)),
            Event::Said(s) if self.node == Node::Choose => self.choose(Place::from_words(&s)),
            Event::Said(s) if wants_help(&s) && matches!(self.node, Node::Check | Node::Guide) => {
                self.guide()
            }
            Event::Said(s)
                if self.node == Node::Check
                    && matches!(self.mission().answer, Answer::Choice(_)) =>
            {
                let choice = match normalized(&s).as_str() {
                    "first" | "the first" | "первая" | "первый" => Some(1),
                    "second" | "the second" | "вторая" | "второй" => Some(2),
                    "same" | "equal" | "the same" | "поровну" | "одинаково" => {
                        Some(3)
                    }
                    _ => None,
                };
                if choice.is_some() {
                    self.answer(choice, false)
                } else {
                    vec![]
                }
            }
            Event::Table(t) => {
                // A moving table that still shows the settled count (a hand pointing at the
                // toys) changes nothing; only a lost or different count does. Pointing used to
                // restart the question on every frame.
                if !t.is_stable && t.count.is_some() && t.count == self.table {
                    return vec![];
                }
                self.table = if t.is_stable { t.count } else { None };
                if self.node == Node::Prepare && self.table == self.target() {
                    self.guide_question()
                } else if self.node == Node::Guide && self.table != self.target() {
                    self.prepare()
                } else {
                    vec![]
                }
            }
            Event::Numbers(ns) if matches!(self.node, Node::Check | Node::Guide) => {
                self.sequence.set(ns);
                vec![]
            }
            Event::Heard(n) if matches!(self.node, Node::Check | Node::Guide) => {
                self.answer(Some(n), false)
            }
            Event::NothingHeard if self.active() && self.node != Node::Prepare => {
                self.misses += 1;
                if self.misses >= 3 {
                    return self.finish(false);
                }
                match self.node {
                    Node::Choose => invitation(&self.config),
                    Node::Guide => self.guide_question(),
                    _ => self.check(),
                }
            }
            Event::RepeatRequested if self.active() => match self.node {
                Node::Choose => invitation(&self.config),
                Node::Prepare => self.prepare(),
                Node::Guide => self.guide_question(),
                _ => self.check(),
            },
            Event::SkipRequested if matches!(self.node, Node::Check | Node::Guide) => {
                self.answer(None, true)
            }
            Event::SkipRequested if self.node == Node::Choose => self.choose(Some(self.place)),
            Event::SkipRequested if self.node == Node::Prepare => self.finish(false),
            _ => vec![],
        }
    }
    fn snapshot(&self) -> StrategySnapshot {
        StrategySnapshot {
            phase: self.phase(),
            level: self.mastered.count_ones(),
            level_count: self.missions.len() as u32 + 1,
            tasks_completed: self.records.len() as u32,
            correct: self.records.iter().filter(|r| r.correct).count() as u32,
            task: Some(format!(
                "{}: {} ({})",
                self.lesson.title(),
                self.mission().name,
                if matches!(self.node, Node::Guide | Node::Prepare) {
                    "guided"
                } else {
                    "independent"
                }
            )),
            target_count: self.target(),
            expected_answer: self.expected(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct AdventureFactory(pub Lesson);
impl StrategyFactory for AdventureFactory {
    fn info(&self) -> StrategyInfo {
        self.0.info()
    }
    fn create(&self, config: &SessionConfig) -> Box<dyn Strategy> {
        Box::new(Adventure::new(self.0, config.clone()))
    }
}
