// SPDX-License-Identifier: LGPL-3.0-or-later
use super::*;
impl Lesson {
    pub fn goal(self) -> &'static str {
        match self {
            Self::CaveTrain => "Drive a toy train and find how many passengers remain after some get off, within five.",
            Self::BridgeBuilders => "Choose more, fewer, or equal supplies for pretend bridges, comparing quantities within five.",
            Self::ParcelDetectives => "Find how many parcels are missing from a delivery of up to five.",
            Self::FireflyGates => "Open pretend firefly gates by counting in order through ten.",
            Self::SupplyLaunch => "Combine cargo loads with totals six through ten to launch supplies.",
            Self::HomewardTrail => "Find how many travelers remain after departures, within ten.",
            Self::TenSeatFerry => "Find the empty seats that complete a ferry of ten.",
            Self::FairShareCafe => "Share even quantities up to ten fairly between two toy guests.",
        }
    }
    pub(super) fn welcome(self) -> Words {
        match self {
            Self::CaveTrain => Words::new("You're our train driver! Toys are passengers. Help them get off at their caves.", "Ты машинист! Игрушки — пассажиры. Помоги им выйти у своих пещер."),
            Self::BridgeBuilders => Words::new("Our friends need bridges! Toys are building blocks. Choose which team has the supplies we need.", "Друзьям нужны мосты! Игрушки — стройматериалы. Выбери команду с нужным запасом."),
            Self::ParcelDetectives => Words::new("Detective, some parcels are missing! Toys are parcels. Can you finish our deliveries?", "Детектив, посылки потерялись! Игрушки — посылки. Помоги собрать заказы."),
            Self::FireflyGates => Words::new("The fireflies hid some pretend gates! Toys are glowing keys. Count to open each gate.", "Светлячки спрятали воображаемые ворота! Игрушки — светящиеся ключи. Считай, чтобы открывать ворота."),
            Self::SupplyLaunch => Words::new("Mission control needs you! Toys are cargo pods. Join the loads so our supplies can launch.", "Центр управления ждёт тебя! Игрушки — грузовые капсулы. Соедини грузы для запуска."),
            Self::HomewardTrail => Words::new("Time for the journey home! Toys are travelers. Some stop to camp; help the others keep going.", "Пора домой! Игрушки — путешественники. Кто-то остаётся в лагере. Посчитай остальных."),
            Self::TenSeatFerry => Words::new("You're the ferry captain! There are ten pretend seats. Find how many friends can still board.", "Ты капитан парома! На нём десять воображаемых мест. Узнай, сколько друзей ещё поместится."),
            Self::FairShareCafe => Words::new("Welcome, cafe chef! Choose two toy guests. Share pretend treats so both get the same amount.", "Добро пожаловать, повар! Выбери двух игрушечных гостей. Раздай воображаемые угощения поровну."),
        }
    }
    pub(super) fn missions(self) -> Vec<Mission> {
        match self {
            Self::CaveTrain => [(3,1),(4,2),(5,1),(2,2),(5,3),(5,0)].into_iter().map(|(a,b)| subtract(a,b,false)).collect(),
            Self::BridgeBuilders => [(2,4,true),(5,3,false),(3,3,true),(4,1,true),(2,5,false),(1,1,false)].into_iter().map(|(a,b,more)| compare(a,b,more)).collect(),
            Self::ParcelDetectives => [(1,3),(2,5),(3,4),(4,5),(0,2),(5,5)].into_iter().map(|(a,total)| missing(a,total,false)).collect(),
            Self::FireflyGates => [6,7,8,9,10,10].into_iter().enumerate().map(|(i,n)| {
                Mission { name: format!("gate {}: count 1..{n}", i+1),
                    question: if i==5 { Words::new("The final gate! Can you count from one to ten again, all by yourself?", "Последние ворота! Посчитай снова от одного до десяти самостоятельно.") } else { Words::new(format!("The firefly gate needs {n} glowing keys! Count from one to {n} to open it."),format!("Воротам светлячков нужно ключей: {n}! Посчитай от одного до {n}, чтобы открыть их.")) },
                    answer: Answer::Sequence(n), stages: vec![stage(n,format!("Pretend these {n} toys are glowing keys. Put them on the table, then move your hands away."),format!("Игрушки — светящиеся ключи. Положи их на стол. Нужно ровно {n}. Убери руки."))],
                    explanation: Words::new(format!("We counted one key for each number, all the way to {n}."),format!("На каждое число мы показали один ключ и досчитали до {n}.")), reward: Words::new("Swing the pretend gate open and flutter like a firefly!", "Открой воображаемые ворота и полетай, как светлячок!") }
            }).collect(),
            Self::SupplyLaunch => [(5,1),(4,3),(5,3),(4,5),(5,5),(6,2)].into_iter().map(|(a,b)| add(a,b)).collect(),
            Self::HomewardTrail => [(6,1),(7,3),(8,2),(9,4),(10,5),(10,10)].into_iter().map(|(a,b)| subtract(a,b,true)).collect(),
            Self::TenSeatFerry => [5,7,3,9,0,10].into_iter().map(|a| missing(a,10,true)).collect(),
            Self::FairShareCafe => [2,4,6,8,10,6].into_iter().enumerate().map(|(i,total)| share(total,i==5)).collect(),
        }
    }
}
fn stage(count: u32, en: String, ru: String) -> Stage {
    Stage {
        count,
        action: Words::new(en, ru),
    }
}
fn subtract(a: u32, b: u32, home: bool) -> Mission {
    let remain = a - b;
    let action = if home {
        "stop to camp"
    } else {
        "get off at the cave"
    };
    Mission { name: format!("{a} - {b}"), question: Words::new(format!("There are {a} travelers. Then {b} {action}. How many are still traveling?"),format!("Ехали путешественники: {a}. На остановке остались: {b}. Сколько едет дальше?")), answer: Answer::Number(remain),
        stages: vec![stage(a,format!("Line up {a} toy travelers on the table. Move your hands away when ready."),format!("Выстрой игрушечных путешественников на столе. Нужно ровно {a}. Убери руки.")), stage(remain,format!("Let {b} toys {action}: take them off the table. Keep the rest here, then move your hands away."),format!("Убери со стола игрушки, которые остались на остановке: {b}. Остальных оставь и убери руки."))],
        explanation: Words::new(format!("We started with {a}, took away {b}, and {remain} stayed. {a} minus {b} is {remain}."),format!("Было {a}, убрали {b}, осталось {remain}. {a} минус {b} равно {remain}.")), reward: if home { Words::new("Make a little tent with your hands. The travelers have a camp!", "Сложи ладошки домиком. У путешественников есть лагерь!") } else { Words::new("Toot toot! Pretend to steer our train to the next cave!", "Ту-ту! Веди воображаемый поезд к следующей пещере!") } }
}
fn add(a: u32, b: u32) -> Mission {
    let total = a + b;
    Mission { name: format!("{a} + {b}"), question: Words::new(format!("One cargo ship brings {a} pods. Another brings {b}. How many pods can we launch altogether?"),format!("Один корабль привёз груз: {a}. Второй привёз ещё: {b}. Сколько всего грузовых капсул отправим?")), answer: Answer::Number(total),
        stages: vec![stage(a,format!("Put {a} toy cargo pods on the table. Move your hands away."),format!("Поставь грузовые капсулы на стол. Нужно ровно {a}. Убери руки.")),stage(total,format!("Bring {b} more toy pods to join them, then move your hands away."),format!("Добавь ещё грузовые капсулы: {b}. Убери руки."))],
        explanation: Words::new(format!("Joining {a} and {b} made {total}. {a} plus {b} is {total}."),format!("Соединили {a} и {b}, получили {total}. {a} плюс {b} равно {total}.")), reward: Words::new("Make a launch sound and lift your pretend cargo ship!", "Издай звук запуска и подними воображаемый корабль!") }
}
fn missing(a: u32, total: u32, ferry: bool) -> Mission {
    let needed = total - a;
    Mission { name: format!("{a} + ? = {total}"), question: if ferry { Words::new(format!("Our ferry has ten seats and {a} passengers. How many more can board to fill it?"),format!("На пароме десять мест. Пассажиров уже: {a}. Сколько ещё нужно, чтобы занять все места?")) } else { Words::new(format!("The delivery needs {total} parcels. We have {a}. How many more parcels do we need?"),format!("Для заказа нужно посылок: {total}. Уже есть: {a}. Сколько ещё нужно?")) }, answer: Answer::Number(needed),
        stages: vec![stage(a,format!("Put our starting group of {a} toys on the table. Move your hands away."),format!("Поставь начальную группу игрушек на стол. Нужно ровно {a}. Убери руки.")),stage(total,format!("Add toys one at a time until there are {total}. Count only the NEW toys as you add them. A grown-up can help. Move your hands away."),format!("Добавляй игрушки по одной, пока всего не станет {total}. Считай только НОВЫЕ игрушки. Взрослый может помочь. Убери руки."))],
        explanation: Words::new(format!("We had {a} and added {needed} new toys to reach {total}. The missing number is {needed}."),format!("Было {a}, добавили {needed}, получили {total}. Недостающее число — {needed}.")), reward: if ferry { Words::new("Ding ding! Wave the pretend ferry away from the shore!", "Динь-динь! Помаши парому, который отплывает от берега!") } else { Words::new("Stamp your pretend parcel and deliver it to a toy friend!", "Поставь воображаемую печать и доставь посылку игрушечному другу!") } }
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
    let solution = if choice == 3 {
        "Both teams have the same amount."
    } else if choice == 1 {
        "The first team is the one we need."
    } else {
        "The second team is the one we need."
    };
    Mission { name: format!("compare {a}, {b}: {kind}"), question: Words::new(format!("First team has {a} blocks. Second team has {b}. Which team has {kind}, or are they the same? Say first, second, or same."),format!("У первой команды блоков: {a}. У второй: {b}. Где {kind_ru}, или поровну? Скажи первая, вторая или поровну.")), answer: Answer::Choice(choice),
        stages: vec![stage(a,format!("Build the first team's supply: put {a} toy blocks on the table. Count them, then move your hands away."),format!("Собери запас первой команды. Положи на стол блоки: {a}. Убери руки.")),stage(b,format!("Now replace them with the second team's {b} blocks. Count them. An adult can help line up two groups off-camera for matching. Move your hands away."),format!("Теперь замени их запасом второй команды: {b}. Взрослый может помочь сопоставить две группы вне кадра. Убери руки."))],
        explanation: Words::new(format!("We compared {a} and {b}. {solution} Pairing one block from each group helps us see what is left."),format!("Сравнили {a} и {b}. Ответ: {}. Сопоставляем блоки по одному из каждой группы и смотрим, остались ли лишние.", if choice==3 {"поровну"} else if choice==1 {"первая команда"} else {"вторая команда"})), reward: Words::new("Stretch your arms into a pretend bridge for our friends!", "Раскинь руки и построй воображаемый мост для друзей!") }
}
fn share(total: u32, transfer: bool) -> Mission {
    let each = total / 2;
    Mission { name: format!("share {total} between 2{}",if transfer {" again"} else {""}), question: Words::new(format!("{}There are {total} treats for two guests. Both must get the same. How many treats should EACH guest get?",if transfer {"Two new guests arrived! "} else {""}),format!("{}Угощений всего: {total}. Гостей двое. Каждому нужно поровну. Сколько получит КАЖДЫЙ?",if transfer {"Пришли два новых гостя! "} else {""})), answer: Answer::Number(each),
        stages: vec![
            stage(total,format!("Put {total} toy treats on the table. Choose two pretend guests outside the camera's toy area, then move your hands away."),format!("Положи игрушечные угощения на стол: {total}. Выбери двух гостей вне области подсчёта камеры. Убери руки.")),
            stage(each,"Give one toy treat to the first guest, one to the second, and repeat until all are shared. Ask a grown-up to help. Put ONLY the first guest's share on the table, then move your hands away.".into(),"Раздавай по одному: сначала первому гостю, потом второму, пока не раздашь всё. Взрослый может помочь. Положи на стол ТОЛЬКО долю первого гостя и убери руки.".into()),
            stage(each,"Now replace those treats with ONLY the second guest's share. Move your hands away, then count that share too.".into(),"Теперь замени угощения на столе ТОЛЬКО долей второго гостя. Убери руки, затем посчитай и эту долю.".into())],
        explanation: Words::new(format!("Give one treat to the first guest, one to the second, and repeat. Ask a grown-up to help check equal groups. With {total} treats, each guest gets {each}."),format!("Дай угощение первому гостю, потом второму. Продолжай по очереди. Попроси взрослого проверить, что поровну. Из {total} угощений каждый получит {each}.")), reward: Words::new("Pretend to serve both guests. What happy munching sounds do they make?", "Угости обоих гостей понарошку. Как они довольно хрумкают?") }
}
