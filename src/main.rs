mod battle;
mod characters;
use characters::*;

fn main() {
    let mut player = character::new();
    let mut enemy = enemies::squirrel();

    battle::fight(&mut player, &mut enemy);
}
