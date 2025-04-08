
fn main(){

use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!

//println!("The field_name and field_value are{:?}",field_name,field_value);//We aren’t able to use the variables 
//field_name and field_value after they’ve been moved into the hash map with the call to insert.

println!("The map values is {:?}",map);

}