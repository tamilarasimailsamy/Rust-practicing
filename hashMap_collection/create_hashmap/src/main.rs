fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("yellow"),50);

    println!("The scores are {:?}",scores);
}
