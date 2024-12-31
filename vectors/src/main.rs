fn main() {
    let mut items = vec!["Sword", "Shield", "Potion"];

    items.push("Boots");
    println!("current items : {:?}", items);
    items.remove(0);
    println!("current items : {:?}", items);

    println!("items len : {}", items.len());
    println!("items capacity : {}", items.capacity());
}
