use std::ops;
use std::fmt;
use std::ops::Mul;

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

#[derive(Debug, Clone, Copy)]
struct B(f32);

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

struct Square<T> {
    x: T
}

fn prnt<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}

struct X;
struct Y;

#[derive(Debug)]
struct XY;
#[derive(Debug)]
struct YX;

impl ops::Add<Y> for X {
    type Output = XY;
    
    fn add(self, _rhs:Y) -> XY {
        XY
    }
}

impl ops::Add<X> for Y {
    type Output = YX;

    fn add(self, _rhs:X) -> YX {
        YX
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

    println!("{:?} {:?} {:?}", a, b, c);

    let s = Square{x: 10};
    let t = Square{x: 1.0};
    let r = Square{x: "Hello"};
    let u = Square{x: 'c'};

    prnt(10);
    prnt(String::from("Some string"));

    println!("{:?}", X + Y);
    println!("{:?}", Y + X);
}
