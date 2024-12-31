use std::{
    sync::{Arc, Mutex},
    thread,
};

// Arc is a thread-safe reference-counting pointer. It is used to share ownership between threads.

// Mutex is a mutual exclusion primitive useful for protecting shared data. Mutex is a smart pointer that wraps the data it protects.

fn main() {
    let crabby_pick_gold = Arc::new(Mutex::new(10));

    let loot_1 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_pick_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();    // unwrap not safe, we should use match expression
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_pick_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();    // unwrap not safe, we should use match expression
            *gold += 200;
        }
    });

    let loot_3 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_pick_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();    // unwrap not safe, we should use match expression
            *gold += 80;
        }
    });


      loot_1.join().unwrap();
      loot_2.join().unwrap();
      loot_3.join().unwrap();
      // wait for all threads to finish


    println!("Gold: {}", crabby_pick_gold.lock().unwrap());


}
