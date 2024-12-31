use std::{cell::RefCell, rc::Rc};

fn main() {
    // let chest = Box::new(10); // sometime we not using Box , or Box will using recursive
    let chest = 10;

    let share_chest: Rc<RefCell<i32>> = Rc::new(RefCell::new(chest));

    let people_1 = Rc::clone(&share_chest);
    let people_2 = Rc::clone(&share_chest);

    *people_1.borrow_mut() += 10;
    *people_2.borrow_mut() += 5;

    // **share_chest.borrow_mut() += 10;
    // **share_chest.borrow_mut() += 5;

    *share_chest.borrow_mut() += 100;
    // *share_chest.borrow_mut() += 5;

    println!("Current chest {}", share_chest.borrow())
}

// # NOTE
// - Box<T> is a smart pointer, Allocates a value on the heap in safe mode
// - Rc<T> is a reference counter, Enables multiple pointers to share ownership.
// - RefCell<T> is a mutable reference, Allows interior mutability.
