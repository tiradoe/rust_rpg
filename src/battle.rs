/*
Battle
    Setup {
       set_order()
       get_user_commands()
    }

    fight()

    End Battle {
        lost() {
            game_over
        }
        won() {
            calculate_experience
            get_reward
            back_to_world
        }
    }
*/

use rand::thread_rng;
use rand::Rng;

pub struct Character {
    pub name: String,
    pub health: u8,
    pub strength: u8,
    pub armor: u8,
}

impl Character {
    fn attack(&self, enemy: &mut Character) {
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
