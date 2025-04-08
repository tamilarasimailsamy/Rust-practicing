fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);//already I have stored blue=10
    scores.insert(String::from("Blue"), 25);//now I have overwriting it to blue=25

    println!("{scores:?}");//it will print blue=25

}
