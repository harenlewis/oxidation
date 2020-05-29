trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

#[derive(Debug, Clone, Copy)]
struct A(i32);

struct B(f32);

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

fn main() {
    println!("Traits / Generics");

    let c = Circle {radius: 100.15};
    let r = Rectangle {x: 10, y: 15};

    println!("{} {}", c.area(), r.area());

    let a = A(32);
    let b = B(10.5);
    let c = a;

    println!("{:?}", a);

}
