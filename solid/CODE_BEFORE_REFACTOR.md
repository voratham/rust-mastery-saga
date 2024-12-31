```rs
struct QuestNotifier;

impl QuestNotifier {
    fn notify(&self, message: &str) {
        println!("Notification: {} ", message);
    }
}

struct QuestManager {
    notifier: QuestNotifier,
}

impl QuestManager {
    fn completed_quest(&self) {
        self.notifier.notify("Quest completed 🎉🎉");
    }
}

fn main() {
    println!("Hello, world!");
}

```