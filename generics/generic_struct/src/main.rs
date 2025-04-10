
//struct Point<T>{
//    x:T,
//    y:T,
//}
//fn main() {
//    let _integer = Point {x:5, y:10};
//    let _float = Point {x:2.3, y:6.3};
//} // leads to error



//struct Point<T> {
 //   x: T,
 //   y: T,
//}

//fn main() {
//    let wont_work = Point { x: 5, y: 4.0 };      // mismatched type error becoz x is integer and y is float
//}


struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}