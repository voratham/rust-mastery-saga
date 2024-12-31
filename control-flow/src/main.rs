// Question1
// fn main() {
//     let weather = "rainy";

//     if weather == "sunny" {
//         println!("Crabby will cross the river by swimming! 🌞")
//     } else if weather == "rainy" {
//         println!("Crabby will build a bridge to stay dry ☔️")
//     } else {
//         println!("Crabby will wait for better weather ")
//     }
// }

// Question2
// fn main() {
//     let monster = "dragon";

//     match monster {
//         "goblin" => println!("Crabby will fight the goblin! ⚔️"),
//         "troll" => println!("Crabby sets a trap! 🏃"),
//         "dragon" => println!("Crabby will run away! 🐉"),
//         _ => println!("Crabby is confused and does nothing! 🤷"),
//     }
// }

// Question3
fn main() {
    let mut wood = 1;

    loop {
        wood += 1;
        println!("Crabby cuts down a tree! then receive 🌲 1 ea");

        if wood == 10 {
            println!("Crabby craft a boat finished! 🚣");
            break;
        }
    }
    println!("Crabby ready to avenues! 🌊");
}
