trait QuestNotifier {
    fn notify(&self, message: &str);
}

struct Pigeon;
impl QuestNotifier for Pigeon {
    fn notify(&self, message: &str) {
        println!("Pigeon: {}", message);
    }
}
struct Email;
impl QuestNotifier for Email {
    fn notify(&self, message: &str) {
        println!("Email: {}", message);
    }
}

struct QuestManager {}

impl QuestManager {
    fn completed_quest<T: QuestNotifier>(&self, notifier: T) {
        notifier.notify("Quest completed! 🎉🎉🎉");
    }
}

fn main() {
    let quest_manager = QuestManager {};

    let pigeon = Pigeon {};
    let email = Email {};

    quest_manager.completed_quest(pigeon);
    quest_manager.completed_quest(email);
}
