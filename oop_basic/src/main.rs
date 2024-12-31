use std::sync::Arc;

trait Weapon {
    fn attack(&self);
}

struct Sword;
struct Staff;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Swing sword");
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Swing staff");
    }
}

struct Warrior {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Warrior {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 10,
            intelligence: 0,
            weapon: Box::new(Sword),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }
        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value)
    }
}

struct Mage {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 0,
            intelligence: 10,
            weapon: Box::new(Staff),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }
        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value)
    }
}

struct Healer {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 5,
            intelligence: 5,
            weapon: Box::new(Staff),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }
        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value)
    }
}

fn special_attack(weapon: Box<dyn Weapon>) {
    weapon.attack()
}

fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();

    println!("Warrior health: {}", warrior.health);
    println!("Mage health: {}", mage.health);
    println!("Healer health: {}", healer.health);

    warrior.health_decrease(10);
    mage.health_decrease(10);
    healer.health_decrease(10);

    println!("After Decrease health - Warrior health: {}", warrior.health);
    println!("After Decrease health - Mage health: {}", mage.health);
    println!("After Decrease health - Healer health: {}", healer.health);

    special_attack(warrior.weapon);
    special_attack(mage.weapon);
    special_attack(healer.weapon);
}
