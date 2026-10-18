// SPDX-License-Identifier: LGPL-3.0-or-later
//! Number agreement for the small concrete quantities spoken in lessons.
fn counted(n: u32, forms: [&str; 3], one: &str, two: &str) -> String {
    let number = match n {
        0 => "ноль".into(),
        1 => one.into(),
        2 => two.into(),
        3 => "три".into(),
        4 => "четыре".into(),
        5 => "пять".into(),
        6 => "шесть".into(),
        7 => "семь".into(),
        8 => "восемь".into(),
        9 => "девять".into(),
        10 => "десять".into(),
        _ => n.to_string(),
    };
    let form = if (11..=14).contains(&(n % 100)) {
        forms[2]
    } else {
        match n % 10 {
            1 => forms[0],
            2..=4 => forms[1],
            _ => forms[2],
        }
    };
    format!("{number} {form}")
}
pub(crate) fn toys(n: u32) -> String {
    counted(n, ["игрушку", "игрушки", "игрушек"], "одну", "две")
}
pub(crate) fn treats(n: u32) -> String {
    counted(n, ["угощение", "угощения", "угощений"], "одно", "два")
}
pub(crate) fn keys(n: u32) -> String {
    counted(n, ["ключ", "ключа", "ключей"], "один", "два")
}
pub(crate) fn pods(n: u32) -> String {
    counted(n, ["капсулу", "капсулы", "капсул"], "одну", "две")
}
pub(crate) fn parcels(n: u32) -> String {
    counted(n, ["посылку", "посылки", "посылок"], "одну", "две")
}
pub(crate) fn blocks(n: u32) -> String {
    counted(n, ["блок", "блока", "блоков"], "один", "два")
}
pub(crate) fn travelers(n: u32) -> String {
    counted(
        n,
        ["путешественник", "путешественника", "путешественников"],
        "один",
        "два",
    )
}
pub(crate) fn passengers(n: u32) -> String {
    counted(n, ["пассажир", "пассажира", "пассажиров"], "один", "два")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoken_quantities_agree_in_gender_case_and_number() {
        for (n, expected) in [
            (0, "ноль игрушек"),
            (1, "одну игрушку"),
            (2, "две игрушки"),
            (3, "три игрушки"),
            (4, "четыре игрушки"),
            (5, "пять игрушек"),
            (10, "десять игрушек"),
        ] {
            assert_eq!(toys(n), expected);
        }
        assert_eq!(treats(1), "одно угощение");
        assert_eq!(treats(2), "два угощения");
        assert_eq!(treats(5), "пять угощений");
        assert_eq!(pods(1), "одну капсулу");
        assert_eq!(pods(2), "две капсулы");
        assert_eq!(parcels(1), "одну посылку");
        assert_eq!(blocks(1), "один блок");
        assert_eq!(keys(6), "шесть ключей");
        assert_eq!(travelers(1), "один путешественник");
        assert_eq!(passengers(0), "ноль пассажиров");
    }
}
