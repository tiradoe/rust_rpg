use rand::thread_rng;
use rand::Rng;

pub struct Character {
    pub name: String,
    pub health: u8,
    pub strength: u8,
    pub armor: u8,
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
