use crate::characters::character::Character;

pub fn fight(hero: &mut Character, enemy: &mut Character) {
    loop {
        println!("\n{}: {}", hero.name, hero.health);
        println!("{}: {}", enemy.name, enemy.health);

        hero.attack(enemy);
        if enemy.health > 0 {
            println!("{} has {} health left.", enemy.name, enemy.health);
        } else {
            println!("\n{} was defeated!", enemy.name);
            end_battle(&hero.name);
            return;
        }

        enemy.attack(hero);
        if hero.health > 0 {
            println!("{} has {} health left.", hero.name, hero.health);
        } else {
            println!("\n{} was defeated!", hero.name);
            end_battle(&enemy.name);
            return;
        }
    }
}

fn end_battle(winner: &String) {
    println!("The fight is over! {} wins!", winner);
}
