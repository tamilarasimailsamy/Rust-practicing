struct Point<X1, Y1> {
    x: X1,// X1 , Y1 are generic parameters in struct
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> { //X2, Y2 are generic parameters of method mixup
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };// x=integer, y=float
    let p2 = Point { x: "Hello", y: 'c' };// x=string slice, y=character

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}