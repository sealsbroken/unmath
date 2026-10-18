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
pub(crate) fn toy_group(n: u32) -> String {
    counted(n, ["игрушка", "игрушки", "игрушек"], "одна", "две")
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
        assert_eq!(toy_group(1), "одна игрушка");
        assert_eq!(toy_group(2), "две игрушки");
        assert_eq!(toy_group(5), "пять игрушек");
    }
}
