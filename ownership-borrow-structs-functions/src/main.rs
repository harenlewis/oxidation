use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

/* Methods */
impl Point {
    fn distance (&self) -> i32 {
        &self.x - &self.y
    }
}

/* Related funcs*/
impl Point {
    fn new (x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Distance between {} and {} is {}", self.x, self.y, self.distance())
    }
}

fn main() {

    println!("Ownership / Borrowing / Structs / Funcs");

    let point = Point {
        x: 10,
        y: 4
    };

    let new_point = Point::new(50, 20);

    println!("{} {} {}", point.x, point.y, point.distance());
    println!("{} {} {}", new_point.x, new_point.y, new_point.distance());

    // with debug trait
    println!("{:?}", new_point);

    println!("{}", new_point);
}
