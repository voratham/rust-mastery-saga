use std::sync::Arc;

use tokio::{
    self,
    sync::mpsc::{self, Sender},
};

async fn gather_herbs() {
    println!("🟢 Crabby is gathering herbs...");
}

async fn collect_gold_coins() {
    println!("🟢 Crabby is collecting gold coins...");
}

async fn fight_monster() {
    println!("🟢 Crabby is fighting a monster !");
}

async fn send_item<'a>(item: &'a str, tx: Arc<Sender<&'a str>>) {
    tx.send(item).await.unwrap();
}

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(gather_herbs());
    let task2 = tokio::spawn(collect_gold_coins());
    let task3 = tokio::spawn(fight_monster());

    let _ = tokio::join!(task1, task2, task3);

    println!("staring mpsc 🚀");
    let items = vec!["herbs", "gold coins", "gems"];

    let (tx, mut rx) = mpsc::channel(items.len());
    let tx_arc = Arc::new(tx);

    for item in items.clone().into_iter() {
        tokio::spawn({
            let tx_arc_clone = Arc::clone(&tx_arc);
            async move {
                send_item(item, tx_arc_clone).await;
            }
        });
    }

    drop(tx_arc);

    for _ in 0..items.len() {
        let result = rx.recv().await.unwrap();
        println!("🦀 Crabby received: {}", result);
    }
}
