fn main(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);//already we have blue (key)=(value) 10

    scores.entry(String::from("Yellow")).or_insert(50);//Yellow team doesn't have value already,so inserting value 50
    scores.entry(String::from("Blue")).or_insert(50);//blue team already have value 10, so ingore inserting 50

    println!("{scores:?}");

}