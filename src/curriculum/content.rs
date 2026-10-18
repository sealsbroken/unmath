// SPDX-License-Identifier: LGPL-3.0-or-later
use super::*;
use crate::play::toy_amount;
use crate::russian::{toy_group, toys};
impl Lesson {
    pub(super) fn correction(self) -> Words {
        match self {
            Self::CaveTrain | Self::HomewardTrail => Words::new("Count only the toys left on the table. Do not count the toys moved aside.", "Посчитай только игрушки, которые остались на столе. Отложенные игрушки не считай."),
            Self::BridgeBuilders => Words::new("Pair one toy from each group. Look for toys without a partner. If none are left, the groups have the same amount.", "Составь пары: одна игрушка из каждой группы. Посмотри, где остались игрушки без пары. Если таких нет, в группах поровну."),
            Self::ParcelDetectives | Self::TenSeatFerry => Words::new("Add toys one at a time to reach the target. Count only the NEW toys, not the ones already there.", "Добавляй игрушки по одной до нужного количества. Считай только НОВЫЕ игрушки, а не те, что уже были."),
            Self::SupplyLaunch => Words::new("Put both toy groups together. Count all the toys, starting at one.", "Соедини обе группы игрушек. Посчитай все игрушки вместе, начиная с одного."),
            Self::FairShareCafe => Words::new("Put one toy in the first group, then one in the second. Repeat until all are shared. Count only ONE group.", "Клади по одной игрушке в первую группу, затем во вторую. Продолжай, пока не разложишь все игрушки. Посчитай только ОДНУ группу."),
            Self::FireflyGates => Words::new("Point to each toy once. Say one number for each, starting at one.", "Показывай на каждую игрушку по очереди и считай. Начни с одного."),
        }
    }
    pub fn goal(self) -> &'static str {
        match self {
            Self::CaveTrain => "Take toys away within five and count what remains.",
            Self::BridgeBuilders => {
                "Compare two groups of toys within five: more, fewer, or equal."
            }
            Self::ParcelDetectives => "Find how many toys to add to reach a target up to five.",
            Self::FireflyGates => "Count toys one by one through ten.",
            Self::SupplyLaunch => "Join toy groups with totals six through ten.",
            Self::HomewardTrail => "Take toys away within ten and count what remains.",
            Self::TenSeatFerry => "Find how many toys to add to make ten.",
            Self::FairShareCafe => "Split even toy groups up to ten equally into two groups.",
        }
    }
    pub(super) fn missions(self) -> Vec<Mission> {
        match self {
            Self::CaveTrain => [(3, 1), (4, 2), (5, 1), (2, 2), (5, 3), (5, 0)]
                .into_iter()
                .map(|(a, b)| subtract(a, b))
                .collect(),
            Self::BridgeBuilders => [
                (2, 4, true),
                (5, 3, false),
                (3, 3, true),
                (4, 1, true),
                (2, 5, false),
                (1, 1, false),
            ]
            .into_iter()
            .map(|(a, b, more)| compare(a, b, more))
            .collect(),
            Self::ParcelDetectives => [(1, 3), (2, 5), (3, 4), (4, 5), (0, 2), (5, 5)]
                .into_iter()
                .map(|(a, t)| missing(a, t))
                .collect(),
            Self::FireflyGates => [6, 7, 8, 9, 10, 10]
                .into_iter()
                .enumerate()
                .map(|(i, n)| Mission {
                    comparison: None,
                    name: format!("count {}: 1..{n}", i + 1),
                    question: Words::new(
                        format!("Count {n} toys one by one, starting at one."),
                        format!("Посчитай игрушки по одной, начиная с одного. Всего их {n}."),
                    ),
                    answer: Answer::Sequence(n),
                    stages: vec![stage(
                        n,
                        format!("Put {n} toys on the table, then move your hands away."),
                        format!("Положи на стол {} и убери руки.", toys(n)),
                    )],
                    explanation: Words::new(
                        "We said one number for each toy.",
                        "Каждой игрушке досталось одно число.",
                    ),
                    reward: reward(),
                })
                .collect(),
            Self::SupplyLaunch => [(5, 1), (4, 3), (5, 3), (4, 5), (5, 5), (6, 2)]
                .into_iter()
                .map(|(a, b)| add(a, b))
                .collect(),
            Self::HomewardTrail => [(6, 1), (7, 3), (8, 2), (9, 4), (10, 5), (10, 10)]
                .into_iter()
                .map(|(a, b)| subtract(a, b))
                .collect(),
            Self::TenSeatFerry => [5, 7, 3, 9, 0, 10]
                .into_iter()
                .map(|a| missing(a, 10))
                .collect(),
            Self::FairShareCafe => [2, 4, 6, 8, 10, 6]
                .into_iter()
                .enumerate()
                .map(|(i, n)| share(n, i == 5))
                .collect(),
        }
    }
}
fn reward() -> Words {
    Words::new("Well done!", "Отлично!")
}
fn stage(count: u32, en: String, ru: String) -> Stage {
    Stage {
        count,
        action: Words::new(en, ru),
    }
}
fn subtract(a: u32, b: u32) -> Mission {
    let remain = a - b;
    let remove = if b == 0 {
        Words::new(
            "Take no toys away. Leave all the toys on the table, then move your hands away.",
            "Не убирай ни одной игрушки. Оставь все игрушки на столе и убери руки.",
        )
    } else {
        Words::new(
            format!(
                "Move {} aside. Leave the others on the table, then move your hands away.",
                toy_amount(b)
            ),
            format!(
                "Отложи {}. Остальные оставь на столе и убери руки.",
                toys(b)
            ),
        )
    };
    Mission { comparison: None, name: format!("{a} - {b}"),
        question: Words::new(format!("Start with {}. If we take {} away, how many toys remain?",toy_amount(a),toy_amount(b)), format!("У нас {}. Если убрать {}, сколько игрушек останется?", toy_group(a), toys(b))),
        answer: Answer::Number(remain),
        stages: vec![stage(a, format!("Put {} on the table. Move your hands away.",toy_amount(a)), format!("Положи на стол {} и убери руки.", toys(a))), Stage { count: remain, action: remove }],
        explanation: Words::new(format!("There were {a} toys. We took {b} away and {remain} remain. {a} minus {b} is {remain}."), format!("Убрали {b} из {a}, осталось {remain}. {a} минус {b} равно {remain}.")), reward: reward() }
}
fn add(a: u32, b: u32) -> Mission {
    let total = a + b;
    Mission {
        comparison: None,
        name: format!("{a} + {b}"),
        question: Words::new(
            format!(
                "One group has {} and another has {}. How many toys are there altogether?",
                toy_amount(a),
                toy_amount(b)
            ),
            format!(
                "В одной группе {}, в другой — {}. Сколько игрушек всего?",
                toy_group(a),
                toy_group(b)
            ),
        ),
        answer: Answer::Number(total),
        stages: vec![
            stage(
                a,
                format!("Put {} on the table. Move your hands away.", toy_amount(a)),
                format!("Положи на стол {} и убери руки.", toys(a)),
            ),
            stage(
                total,
                format!(
                    "Add {} to the group, then move your hands away.",
                    toy_amount(b)
                ),
                format!("Добавь ещё {} и убери руки.", toys(b)),
            ),
        ],
        explanation: Words::new(
            format!(
                "Joining {} and {} made {total}. {a} plus {b} is {total}.",
                toy_amount(a),
                toy_amount(b)
            ),
            format!("Соединили {a} и {b}, получили {total}. {a} плюс {b} равно {total}."),
        ),
        reward: reward(),
    }
}
fn missing(a: u32, total: u32) -> Mission {
    let needed = total - a;
    Mission { comparison:None,name:format!("{a} + ? = {total}"),
        question:Words::new(format!("We need {}. We have {a}. How many more toys do we need?",toy_amount(total)),format!("Нужно собрать {}. Уже есть {}. Сколько игрушек нужно добавить?",toys(total),toy_group(a))),
        answer:Answer::Number(needed),
        stages:vec![stage(a,format!("Start with {} on the table. Move your hands away.",toy_amount(a)),if a==0 {"Сначала убери все игрушки со стола и убери руки.".into()} else {format!("Сначала положи на стол {} и убери руки.",toys(a))}),
            stage(total,format!("Add toys one at a time until there are {total}. Count only the NEW toys as you add them. Move your hands away."),format!("Добавляй игрушки по одной, пока всего не станет {total}. Считай только НОВЫЕ игрушки. Убери руки.")),
            stage(needed,"Keep ONLY the NEW toys on the table. Put the original toys aside. Count the new toys, not the whole group.".into(),"Оставь на столе только НОВЫЕ игрушки. Те, что были раньше, отложи. Посчитай новые игрушки.".into())],
        explanation:Words::new(format!("We started with {} and added {needed} to reach {total}.",toy_amount(a)),format!("Было {a}, добавили {needed}, получили {total}.")), reward:reward() }
}
fn compare(a: u32, b: u32, more: bool) -> Mission {
    let choice = if a == b {
        3
    } else if (a > b) == more {
        1
    } else {
        2
    };
    let kind = if more { "MORE" } else { "FEWER" };
    let kind_ru = if more { "БОЛЬШЕ" } else { "МЕНЬШЕ" };
    Mission { comparison:Some(more),name:format!("compare {a}, {b}: {kind}"),
        question:Words::new(format!("The first group has {}. The second has {}. Which group has {kind}, or are they the same? Say first, second, or same.",toy_amount(a),toy_amount(b)),format!("В первой группе {}. Во второй — {}. В какой группе игрушек {kind_ru}? Или в обеих поровну? Скажи: первая, вторая или поровну.",toy_group(a),toy_group(b))),
        answer:Answer::Choice(choice),
        stages:vec![stage(a,format!("Put {} on the table for the first group. Count them, then move your hands away.",toy_amount(a)),format!("Положи на стол {} для первой группы и убери руки.",toys(a))),
            stage(b,format!("Put the first group beside the table. Put {} on the table for the second group. Count them, then move your hands away.",toy_amount(b)),format!("Отложи первую группу рядом со столом. Положи на стол {} для второй группы и убери руки.",toys(b)))],
        explanation:Words::new("We matched one toy from each group. Toys without a partner show which group has more. If every toy has a partner, the groups are equal.","Мы составили пары: одна игрушка из каждой группы. Игрушки без пары показывают, где их больше. Если у каждой игрушки есть пара, то поровну."),reward:reward() }
}
fn share(total: u32, transfer: bool) -> Mission {
    let each = total / 2;
    Mission { comparison:None,name:format!("share {total} between 2{}",if transfer {" again"} else {""}),
        question:Words::new(format!("Share {total} toys equally between two groups. How many toys should EACH group have?"),format!("Раздели {} поровну на две группы. Сколько игрушек будет в КАЖДОЙ группе?",toys(total))),
        answer:Answer::Number(each),
        stages:vec![stage(total,format!("Put {total} toys on the table. Move your hands away."),format!("Положи на стол {} и убери руки.",toys(total))),
            stage(each,"Place one toy in the first group and one in the second, taking turns until all are shared. Put ONLY the first group on the table, then move your hands away.".into(),"Клади по одной игрушке в первую группу, затем во вторую, пока не разложишь все игрушки. Положи на стол ТОЛЬКО первую группу и убери руки.".into()),
            stage(each,"Put the first group aside. Put ONLY the second group on the table, then move your hands away and count it.".into(),"Отложи первую группу. Положи на стол ТОЛЬКО вторую группу. Убери руки и посчитай игрушки.".into())],
        explanation:Words::new(format!("Taking turns made two equal groups. Each group has {}.",toy_amount(each)),format!("Мы раскладывали игрушки по очереди и получили две равные группы. В каждой группе {}.",toy_group(each))),reward:reward() }
}
impl Mission {
    pub(super) fn error(&self, lesson: Lesson, answer: Option<u32>) -> Words {
        let a = self.stages[0].count;
        match lesson {
            Lesson::CaveTrain|Lesson::HomewardTrail if answer==Some(a) => Words::new("That counts the toys before we moved any aside. Count only the toys still on the table.","Это количество игрушек до того, как мы часть отложили. Посчитай только те, что остались на столе."),
            Lesson::CaveTrain|Lesson::HomewardTrail if answer==Some(a-self.answer.expected()) => Words::new("That counts the toys moved aside. We need the ones still on the table. Keep the groups separate.","Это количество отложенных игрушек. Нам нужны те, что остались на столе. Разделим эти группы."),
            Lesson::BridgeBuilders if self.answer.expected()==3 => Words::new("Let's check for equal groups. Make pairs, one toy from each group. If none are left without partners, say same.","Проверим, поровну ли игрушек. Составим пары: одна игрушка из каждой группы. Если игрушек без пары нет, скажем: поровну."),
            Lesson::BridgeBuilders if self.comparison==Some(false) => Words::new("We need the group with FEWER toys. The row with toys left without partners has MORE. Count and match the rows to find the smaller group.","Нам нужна группа, где игрушек МЕНЬШЕ. Там, где остались игрушки без пары, их БОЛЬШЕ. Посчитаем игрушки и составим пары, чтобы найти меньшую группу."),
            Lesson::ParcelDetectives|Lesson::TenSeatFerry if self.answer.expected()==0 => Words::new("We already have the target number of toys. No new toys are needed. Let's check the group.","Мы уже собрали нужное количество игрушек. Новые игрушки не нужны. Проверим группу."),
            Lesson::ParcelDetectives|Lesson::TenSeatFerry if answer==Some(self.stages[1].count) => Words::new("That is the whole group. We need only the NEW toys. Keep the original toys and the new toys separate.","Это вся группа. Нам нужны только НОВЫЕ игрушки. Отделим те, что были раньше, от тех, что добавились."),
            Lesson::ParcelDetectives|Lesson::TenSeatFerry if answer==Some(a) => Words::new("Those toys were here before. We need to find how many NEW toys to add.","Эти игрушки были здесь раньше. Мы ищем, сколько НОВЫХ игрушек нужно добавить."),
            Lesson::SupplyLaunch if answer==Some(a)||answer==Some(self.stages[1].count-a) => Words::new("That counts just one group. Let's join both toy groups and count all the toys.","Так мы посчитали только одну группу. Соединим обе группы игрушек и посчитаем все игрушки."),
            Lesson::FairShareCafe if answer==Some(a) => Words::new("That counts all the toys. We need the number in ONE group. Put the toys into groups in turns.","Это все игрушки вместе. А мы ищем, сколько игрушек в ОДНОЙ группе. Разложим игрушки по очереди."),
            Lesson::FairShareCafe if answer==Some(2) => Words::new("There are two groups. Count the toys in ONE group, not the groups themselves.","Групп две. Нужно посчитать игрушки в ОДНОЙ группе, а не сами группы."),
            _=>lesson.correction(),
        }
    }
    pub(super) fn followup(&self, lesson: Lesson) -> Words {
        match lesson {
            Lesson::BridgeBuilders => if self.comparison==Some(true) {
                Words::new("Look at the pairs. We need MORE toys. Which group should we choose? Say first, second, or same.","Посмотри на пары. Нам нужно БОЛЬШЕ игрушек. Какую группу выберем? Скажи: первая, вторая или поровну.")
            } else {
                Words::new("Look at the pairs. We need FEWER toys. Which group should we choose? Say first, second, or same.","Посмотри на пары. Нам нужно МЕНЬШЕ игрушек. Какую группу выберем? Скажи: первая, вторая или поровну.")
            },
            Lesson::CaveTrain|Lesson::HomewardTrail => Words::new("Look only at the toys still on the table. How many remain?","Посмотри только на игрушки, которые остались на столе. Сколько их?"),
            Lesson::ParcelDetectives|Lesson::TenSeatFerry => Words::new("Think about the NEW toys. How many toys did we need to add?","Вспомни НОВЫЕ игрушки. Сколько игрушек нужно было добавить?"),
            Lesson::SupplyLaunch=>Words::new("Both toy groups are together now. How many toys are there altogether?","Теперь обе группы игрушек вместе. Сколько игрушек всего?"),
            Lesson::FairShareCafe=>Words::new("The toys are in two groups now. How many toys are in EACH group?","Теперь игрушки в двух группах. Сколько игрушек в КАЖДОЙ группе?"),
            Lesson::FireflyGates=>Words::new("Point to each toy once and count all the toys again, starting at one.","Покажи на каждую игрушку по очереди и посчитай все игрушки ещё раз. Начни с одного."),
        }
    }
}
