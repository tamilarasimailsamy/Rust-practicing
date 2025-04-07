fn main(){

let v = vec![1,4,2,5,6,9];


//let third: &i32 = &v[2];

//println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
}