enum CrabbyState {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyState {
    fn state_present(&self) {
        match self {
            CrabbyState::Fighting => println!("Crabby is fighting"),
            CrabbyState::Collecting(amount) => println!("Crabby is collecting {}", amount),
            CrabbyState::Defending => println!("Crabby is defending"),
        }
    }
}

fn main() {
    let fighting = CrabbyState::Fighting;
    let collection = CrabbyState::Collecting(20);
    let defending = CrabbyState::Defending;

    fighting.state_present();
    collection.state_present();
    defending.state_present();
}
