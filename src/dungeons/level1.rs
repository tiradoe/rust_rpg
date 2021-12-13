use crate::battle;
use crate::characters::*;

pub fn start(mut player: character::Character) -> character::Character {
    let description: String = String::from("Welcome to the forest!");
    println!("{}", description);

    let mut enemy = enemies::squirrel();

    battle::fight(&mut player, &mut enemy);
    player
}
