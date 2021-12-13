use rand::thread_rng;
use rand::Rng;
use std::io;

pub struct Character {
    pub name: String,
    pub health: u8,
    pub strength: u8,
    pub armor: u8,
    pub class: String,
}

impl Character {
    pub fn attack(&self, enemy: &mut Character) {
        let mut rng = thread_rng();
        let damage = rng.gen_range(0, self.strength);

        println!(
            "\n{} attacks {}! {0} deals {} damage!",
            self.name, enemy.name, damage
        );

        let health: u8 = match enemy.health.checked_sub(damage) {
            Some(result) => result,
            None => 0,
        };
        enemy.health = health;
    }
}

pub fn new() -> Character {
    let mut name: String;

    loop {
        println!("What is your character's name?");
        name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input.");

        name = String::from(name.trim());

        if name == "" {
            continue;
        } else {
            break;
        }
    }

    Character {
        name,
        health: 30,
        strength: 7,
        armor: 12,
        class: String::from("Knight"),
    }
}
