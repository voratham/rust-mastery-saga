fn main() {
    let treasures = [
        "Gold",
        "Silver",
        "Diamonds",
        "Ruby Gems",
        "Emeralds",
        "Emeralds",
        "Emeralds",
    ];

    let mut energy = 5;

    for treasure in treasures.iter() {
        println!("current you are energy :: {}", energy);
        if energy == 0 {
            println!("You are out of energy. Game over ! 🔴");
            break;
        } else if treasure == &"Crystal" {
            println!("You found the {} 💚", treasure);
            break;
        }

        energy -= 1;
    }

    println!("The game is ended");
}
