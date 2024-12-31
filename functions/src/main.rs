fn main() {
    let result = crabby_tasks("gathering coins", 12);
    println!("{}", result);

    let result_2 = crabby_tasks("Cooking", 30);
    println!("{}", result_2);

    let result_3 = crabby_tasks("Hunting", 60);
    println!("{}", result_3);
}

fn crabby_tasks(task: &str, time: i32) -> String {
    format!("Crabby has successfully done {} in {} minutes", task, time)
}
