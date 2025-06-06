// fn largest_i32(list: &[i32]) -> &i32 { //integer data type
//    let mut largest = &list[0];

//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }

//    largest
//}

//fn largest_char(list: &[char]) -> &char { // char data type
//    let mut largest = &list[0];

//    for item in list {
//        if item > largest {
//            largest = item;
//        }
  //  }

//    largest
//}

//fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];

//    let result = largest_i32(&number_list);
//    println!("The largest number is {result}");

//    let char_list = vec!['y', 'm', 'a', 'q'];

//    let result = largest_char(&char_list);
//    println!("The largest char is {result}");
//}

// we can use generic in this function

fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

