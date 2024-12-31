use std::collections::HashMap;

fn main() {
    let mut inventory: HashMap<&str, i32> = HashMap::new();

    inventory.insert("Sword", 1);
    inventory.insert("Bow", 1);
    inventory.insert("Arrow", 200);

    if let Some(arrow) = inventory.get_mut("Arrow") {
        *arrow += 100;
    }

    let axe = inventory.get("Axe").unwrap_or(&10);
    println!("axe : {}", axe); // but axe is not in the inventory
    println!("Inventory: {:#?}", inventory);
}
