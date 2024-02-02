#[derive(Debug)]
enum Level {
    Debug,
    Info,
    Warn,
    Err,
}

#[derive(Debug)]
enum Cell {
    Integer(i64),
    Text(String),
    Float(f64),
}

impl Cell {
    fn display(&self) -> () {
        match self {
            Cell::Integer(n) => println!("Cell::Integer({n})"),
            Cell::Text(ref s) => println!("Cell::Text({s})"),
            Cell::Float(f) => println!("Cell::Float({f})"),
        }
    }
}

fn main() {
    let level1 = Level::Debug;
    // println!("level1 = {}", level1); // `Level` doesn't implement `std::fmt::Display`
    println!("level1 = {:?}", level1);
    println!("level1 = {:#?}", level1);

    let cell1 = Cell::Integer(101);
    let mut cell2 = Cell::Text(String::from("liu"));
    match &mut cell2 {
        Cell::Text(text) => text.push_str("..."),
        _ => (),
    };
    let mut cell3 = Cell::Float(1991.5);
    match cell3 {
        Cell::Float(ref mut f) => *f += 100.0,
        _ => (),
    }
    println!(
        "cell1 = {:#?}, cell2 = {:#?}, cell3 = {:#?}",
        cell1, cell2, cell3
    );

    let row: [Cell; 3] = [
        Cell::Integer(1002),
        Cell::Text(String::from("liu")),
        Cell::Float(100.52),
    ];
    println!("row = {:?}", row);
    row[0].display();

    let mut a: Option<String> = Option::None;
    println!("a = {:?}", a);
    if 100 > 10 {
        a = Option::Some(String::from("ok"));
    }
    // match a {
    //     None => println!("a is NULL"),
    //     Some(s) => println!("a = {:?}", s),
    // }
    // println!("a = {:?}", a); // borrow of partially moved value: `a`

    match a {
        None => println!("=> a is None"),
        Some(ref s) => println!("=> a is Some({:?})", s),
    };
    println!("a = {:?}", a);

    if let Option::Some(s) = a {
        println!("s = {:?}", s);
    };
}
