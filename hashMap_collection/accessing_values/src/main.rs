fn main(){
    use std::collections::HashMap;

    let mut scores=HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("green"),30);
    for(key,values)in &scores{
        println!("{key}:{values}");
    }
}