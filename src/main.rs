pub mod battle;
use battle::Character;

fn main() {
    let mut player = Character {
        name: String::from("Edward"),
        health: 30,
        strength: 6,
        armor: 12,
    };

    let mut enemy = Character {
        name: String::from("Shit Knight"),
        health: 30,
        strength: 7,
        armor: 12,
    };

    battle::fight(&mut player, &mut enemy);
}
