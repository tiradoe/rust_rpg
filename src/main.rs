mod battle;
mod characters;
mod dungeons;

use dungeons::*;

fn main() {
    let mut player = characters::character::new();

    player = level1::start(player);

    println!("\n{} has {} health.", player.name, player.health);
}
