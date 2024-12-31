// Declarative Macros Example
macro_rules! greet {
    () => {
        println!("Hello from greet marco");
    };
    (crabby) => {
        println!("Hello from greet marco with crabby");
    };
    ($name:expr) => {
        println!("Hello from greet marco with name: {}", $name)
    }; // ($name:ident) => {
       //     println!("Hello from greet marco with name: {}", stringify!($name));
       // };
}

// https://doc.rust-lang.org/reference/macros-by-example.html

macro_rules! magic_spelling {
    (fire) => {
        println!("You cast a fireball! 🔥🔥🔥");
    };
    (water) => {
        println!("You cast a waterball! 💧💧💧");
    };
    (wind) => {
        println!("You cast a win cut! 💨💨💨");
    };
}

use log_time::log_time;

#[log_time]
fn main() {
    greet!();
    greet!(crabby);
    greet!("test");
    // greet!(world);

    magic_spelling!(fire);
    magic_spelling!(water);
    magic_spelling!(wind);
}
