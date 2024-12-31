trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("🟢 Swing Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("🟢 Fire Arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("🟢 Drink Potion");
    }
}

// using static dispatch (Generic) , increase performance
// fn use_gear<T: Gear>(item: T) {
//     item.use_gear();
// }

// using dynamic dispatch borrow dyn
// fn use_gear(item: &dyn Gear) {
//     item.use_gear();
// }

// using dynamic dispatch with BOX
// ถ้าไม่มี return ก็ไม่จำเป็นต้องใช้ Box
fn use_gear(item: Box<dyn Gear>) {
    item.use_gear();
}

// we cannot return interface type, because rust will eventually need to know the size of the type
// fn get_item(item_type: &str) -> Gear {
//     match item_type {
//         "sword" => Sword,
//         "bow" => Bow,
//         "potion" => Potion,
//         _ => panic!("Invalid item type"),
//     }
// }
// How to solve - dyn บอกให้ rust ไปคำนวณ size ของ type ที่เป็น dynamic dispatch ใน runtime
fn get_item(item_type: &str) -> Box<dyn Gear> {
    match item_type {
        "sword" => Box::new(Sword),
        "bow" => Box::new(Bow),
        "potion" => Box::new(Potion),
        _ => panic!("Invalid item type"),
    }
}

fn main() {
    // let crabby_with_sword: Sword = Sword;
    // let crabby_with_bow: Bow = Bow;
    // let crabby_with_potion: Potion = Potion;

    let crabby_with_sword = Box::new(Sword);
    let crabby_with_bow = Box::new(Bow);
    let crabby_with_potion = Box::new(Potion);

    // use_gear(&crabby_with_sword);
    // use_gear(&crabby_with_bow);
    // use_gear(&crabby_with_potion);

    use_gear(crabby_with_sword);
    use_gear(crabby_with_bow);
    use_gear(crabby_with_potion);
}
