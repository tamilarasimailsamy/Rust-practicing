// we can use the method in struct and enum

struct Point<T> { 
    x: T,
    y: T,
}

impl<T> Point<T> { //impl point<i32> (example)
    fn x(&self) -> &T { //fn distance_from_origin(&self)-> f32{
        &self.x // (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}