fn main() {
    let x: i32 = 1;
    let y = 0.5;
    let z = x + y as i32;

    let msg = String::from("Hello, World!");
    let msg_2 = "Hello, World!".to_string();
    let msg_3 = "Hello, World";
    let msg_4 = format!("{} , {}", x, y);

    println!("Result : {}" , msg_4)
     
}
