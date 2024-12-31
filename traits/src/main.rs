struct Inventory<T> {
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T>
where
    T: std::fmt::Debug,
{
    fn display(&self) {
        println!("{:#?}", self.item);
    }
}

fn main() {
    let gold = Inventory {
        item: String::from("Gold coin"),
    };

    gold.display();

    let arrow = Inventory {
        item: vec!["FireArrow", "IceArrow", "LightningArrow"],
    };

    arrow.display();
}
