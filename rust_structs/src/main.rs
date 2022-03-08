struct Soldier {
    name: String,
    health: isize,
    damage: isize,
}

impl Soldier {
    fn new(name: String, health: isize, damage: isize) -> Soldier {
        Soldier {
            name,
            health,
            damage,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> isize {
        self.health
    }

    fn get_damage(&self) -> isize {
        self.damage
    }

    fn can_beat(&self, other: &Soldier) -> bool {
        self.get_damage() >= other.get_health()
    }
}

fn main() {
    let adam = Soldier::new("Adam".to_string(), 100, 50);
    let sabrina = Soldier::new("Sabrina".to_string(), 51, 52);

    let can_adam_beat_sabrina = adam.can_beat(&sabrina);
    let can_sabrina_beat_adam = sabrina.can_beat(&adam);
    let can_sabrina_beat_sabrina = sabrina.can_beat(&sabrina);

    println!("Fight: {} vs {}", adam.get_name(), sabrina.get_name());
    println!("Is adam able to beat sabrina? {}", can_adam_beat_sabrina);
    println!("Is sabrina able to beat adam? {}", can_sabrina_beat_adam);
    println!("Is sabrina able to beat sabrina? {}", can_sabrina_beat_sabrina);
}
