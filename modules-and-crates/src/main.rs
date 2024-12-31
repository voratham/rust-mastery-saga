// step 0
// mod potions {
//     pub fn use_item() {
//         println!("✅ You used a potion!");
//     }
// }

mod weapons {
    pub fn use_item() {
        println!("✅ You used a weapon!");
    }
}

mod map {
    pub fn use_item() {
        println!("✅ You used a map!");
    }
}

// step 1 and step2
// use modules_and_crates::potions;
// step 3
use modules_and_crates::potions;

fn main() {
    
    potions::use_item::use_item();
    weapons::use_item();
    map::use_item();
}
