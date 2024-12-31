fn main() {
    let old_map = String::from("Old map");

    let borrow_old_map = old_map.as_str(); // or &old_map

    let mut crabby_pick_map = borrow_old_map.to_string(); // take ownership of the string

    crabby_pick_map.push_str("Of crabby");

    println!("old_map : {}", old_map);

    println!("crabby_pick_map: {}", crabby_pick_map);
}
