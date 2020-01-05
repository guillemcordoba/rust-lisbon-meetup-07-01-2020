fn main() {
    println!("Hello, world!");

    let mage = Mage {
        name: String::from("Gandalf"),
        knownledge: 10,
        spell: Spell::Fireball,
    };

    mage.fight();
}

#[derive(Debug)]
enum Spell {
    Fireball,
    Iceray,
}

pub struct Mage {
    name: String,
    knownledge: usize,
    spell: Spell,
}

trait Fighter {
    fn level(self) -> usize;
    fn fight(self) -> ();
}

impl Fighter for Mage {
    fn level(self) -> usize {
        self.knownledge
    }

    fn fight(self) -> () {
        println!("I throw a spell {:?} at you", self.spell);
    }
}
