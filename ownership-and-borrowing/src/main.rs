fn main() {
    let mut treasure = String::from("Gold coins");

    //  multiple friends borrow immutably!
    let friend1 = &treasure;
    let friend2: &String = &treasure;

    println!("Friend 1 sees: {}", friend1);
    println!("Friend 1 sees: {}", friend2);

    // Trusted friend borrows mutably
    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" and silver coins");

    println!("Trusted friend updates: {}", trusted_friend);
}
