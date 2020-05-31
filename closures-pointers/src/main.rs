enum List {
    Cons(i32),
    End,
}

use List::Cons;
use List::End;

fn main() {
    println!("Closures / Box Pointer / Iterators");

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
}
