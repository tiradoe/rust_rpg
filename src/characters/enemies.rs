use crate::characters::character::Character;

pub fn squirrel() -> Character {
    Character {
        name: String::from("Squirrel"),
        armor: 12,
        class: String::from("animal"),
        health: 20,
        strength: 8,
    }
}
