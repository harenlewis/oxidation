fn main() {
    println!("Flow controls / Conditionals / Pattern matching");

    let x = 2;

    match x {
        1 => println!("Its a one"),
        2 => println!("Its a two"),
        3 => println!("Its a three"),
        _ => println!("Its a wrong number"),
    }

    match x {
        1 | 2 => println!("Its a one"),
        3..=8 => println!("{}", x),
        _ => println!("Its a wrong number"),
    }

    let pair = (0, -2);

    match  pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _      => println!("Nope")
    }

    let p = 10;

    let n = match p {
        n @ 1 ..= 5 => n,
        n @ 6 ..= 9 => n *2,
        _ => 0,
    };

    println!("{}", n)
}
