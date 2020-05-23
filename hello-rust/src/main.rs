use std::mem;

fn main() {
    println!("Hello, world!");
    println!("Some cool print statement");

    let x = 123;
    println!("{}", x);

    let a: [i32; 3] = [1, 2, 3] ;
    println!("{:?} {} {}", a, a[2], mem::size_of_val(&a));

    let s = "Slice string";
    println!("{}", s);
}
