use crate::characters::character::Character;

pub fn player() -> Character {
    Character {
        name: String::from("Edward"),
        health: 30,
        strength: 7,
        armor: 12,
    }
}
