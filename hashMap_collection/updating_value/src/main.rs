//Updating a Value Based on the Old Value

fn main() {
    use std::collections::HashMap;
    let text="Hello world is a wonderful world";
    let mut map=HashMap::new();

    for word in text.split_whitespace(){//split_whitespace will seperate by space in text
        let count=map.entry(word).or_insert(0);//or_insert method returns mutable value, entry keyword to check the words
        *count += 1;//mutable reference values stored in count variable,* makes the values dereference

        println!("{map:?}");
    }
}
