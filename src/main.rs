use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please provide an action");
    let item = std::env::args().nth(2).expect("Please provide an item");

    let data = format!("{}: {}", action, item);
    match std::fs::write("./db.txt", data) {
        Err(why) => panic!("An error occurred: {}", why),
        Ok(_) => println!("File successfully written"),
    };

}

struct Todo {
    map: HashMap<String, bool>,
}