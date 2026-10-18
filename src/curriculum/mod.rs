// SPDX-License-Identifier: LGPL-3.0-or-later
//! Lessons 3–10 share event plumbing, while their questions and physical actions differ.
mod content;
mod runtime;
pub use runtime::{Adventure, AdventureFactory};
use std::sync::Arc;
use un_core::{language::Language, strategy::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lesson {
    CaveTrain,
    BridgeBuilders,
    ParcelDetectives,
    FireflyGates,
    SupplyLaunch,
    HomewardTrail,
    TenSeatFerry,
    FairShareCafe,
}
pub const NEXT_LESSONS: [Lesson; 8] = [
    Lesson::CaveTrain,
    Lesson::BridgeBuilders,
    Lesson::ParcelDetectives,
    Lesson::FireflyGates,
    Lesson::SupplyLaunch,
    Lesson::HomewardTrail,
    Lesson::TenSeatFerry,
    Lesson::FairShareCafe,
];
impl Lesson {
    pub fn number(self) -> u32 {
        self as u32 + 3
    }
    pub fn id(self) -> &'static str {
        match self {
            Self::CaveTrain => "unmath-subtract-to-five",
            Self::BridgeBuilders => "unmath-compare-to-five",
            Self::ParcelDetectives => "unmath-missing-to-five",
            Self::FireflyGates => "unmath-count-to-ten",
            Self::SupplyLaunch => "unmath-add-to-ten",
            Self::HomewardTrail => "unmath-subtract-to-ten",
            Self::TenSeatFerry => "unmath-make-ten",
            Self::FairShareCafe => "unmath-share-between-two",
        }
    }
    pub fn title(self) -> &'static str {
        match self {
            Self::CaveTrain => "Lesson 3: cave train",
            Self::BridgeBuilders => "Lesson 4: bridge builders",
            Self::ParcelDetectives => "Lesson 5: parcel detectives",
            Self::FireflyGates => "Lesson 6: firefly gates",
            Self::SupplyLaunch => "Lesson 7: supply launch",
            Self::HomewardTrail => "Lesson 8: homeward trail",
            Self::TenSeatFerry => "Lesson 9: ten-seat ferry",
            Self::FairShareCafe => "Lesson 10: fair-share cafe",
        }
    }
    pub fn required_toys(self) -> u32 {
        if self.number() <= 5 {
            5
        } else {
            10
        }
    }
    pub fn mission_count(self) -> usize {
        self.missions().len()
    }
    pub fn info(self) -> StrategyInfo {
        StrategyInfo {
            id: self.id().into(),
            title: self.title().into(),
            description: self.goal().into(),
            languages: vec![Language::English, Language::Russian],
            license: "LGPL-3.0-or-later".into(),
        }
    }
    fn key(self) -> String {
        format!("unmath.lesson{}.adventure.v1", self.number())
    }
}
pub fn register(registry: &mut StrategyRegistry) {
    for lesson in NEXT_LESSONS {
        registry.register(Arc::new(AdventureFactory(lesson)));
    }
}
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::CaveTrain))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::BridgeBuilders))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::ParcelDetectives))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::FireflyGates))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::SupplyLaunch))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::HomewardTrail))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::TenSeatFerry))) }
inventory::submit! { LinkedStrategy(|| Arc::new(AdventureFactory(Lesson::FairShareCafe))) }

#[derive(Clone)]
struct Words {
    en: String,
    ru: String,
}
impl Words {
    fn new(en: impl Into<String>, ru: impl Into<String>) -> Self {
        Self {
            en: en.into(),
            ru: ru.into(),
        }
    }
    fn in_language(&self, language: Language) -> String {
        match language {
            Language::English => self.en.clone(),
            Language::Russian => self.ru.clone(),
        }
    }
}
#[derive(Clone)]
enum Answer {
    Number(u32),
    Sequence(u32),
    Choice(u32),
}
impl Answer {
    fn expected(&self) -> u32 {
        match *self {
            Self::Number(n) | Self::Sequence(n) | Self::Choice(n) => n,
        }
    }
}
#[derive(Clone)]
struct Stage {
    count: u32,
    action: Words,
}
#[derive(Clone)]
struct Mission {
    comparison: Option<bool>,
    name: String,
    question: Words,
    answer: Answer,
    stages: Vec<Stage>,
    explanation: Words,
    reward: Words,
}
