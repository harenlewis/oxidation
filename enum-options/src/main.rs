enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Passed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Passed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Passed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Passed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    println!("Enum / Options");

    let u = Direction::Up(Point {x:10, y:5});
    let k = u.match_direction();
    let x = k.destruct();
    println!("{:?}", k);
    println!("{:?}", x);

}
