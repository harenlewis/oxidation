use std::collections::HashMap;
use std::fs::File;

fn main() {
    println!("Vectors / Hashmaps / Casting");

    let x = vec![1, 2, 3];
    println!("{:?} {:?} {:?}", x, x.len(), x.capacity());

    let mut y = Vec::new();
    y.push(99);
    y.push(98);
    y.push(97);


    println!("{:?} {:?} {:?}", y, y.len(), y.capacity());
    
    let mut y = Vec::new();
    y.push(99);
    y.push(98);
    y.push(97);


    println!("{:?} {:?} {:?}", y, y.len(), y.capacity());

    println!("{:?}", y.pop());

    let mut hm = HashMap::new();

    hm.insert(String::from("wow"), 1);
    hm.insert(String::from("some"), 2);
    hm.insert(String::from("test"), 3);

    println!("{:?}", hm);

    for (k, v) in &hm {
        println!("{} ===> {}", k, v)
    }

    match hm.get(&String::from("wow")) {
        Some(&n) => println!("Value fetched {}", n),
        _        => println!("No value found!")
    }

    let test = Some(20);

    if let Some(so) = test {
        println!("{:?}", so)
    }

    let file = File::open("test.txt");

    let _f = match file {
        Ok(file) => file,
        Err(error) => panic!("Error while opening file {}", error),
    };
}
