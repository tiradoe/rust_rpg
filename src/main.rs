mod battle;
mod characters;

use characters::enemies;
use characters::heroes;

fn main() {
    let mut player = heroes::player();
    let mut enemy = enemies::shit_knight();

    battle::fight(&mut player, &mut enemy);
}
