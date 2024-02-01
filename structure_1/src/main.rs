use rand::Rng;

#[derive(Debug)]
struct User {
    pub id: usize,
    pwd: String,
    pub name: String,
    pub desc: String,
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Fuck();

fn main() {
    let u1 = User {
        id: 1001,
        pwd: String::from("12345678"),
        name: String::from("liu"),
        desc: String::from("good boy"),
    };
    // println!("u1 = {}", u1); // `User` doesn't implement `std::fmt::Display`
    println!("u1 = {:?}", u1);
    println!("u1 = {:#?}", u1);
    println!(
        "u1 = {{id: {}, pwd: {}, name: {}, desc: {}}}",
        u1.id, u1.name, u1.pwd, u1.desc
    );
    // u1.id = 1010; // cannot assign to `u1.id`, as `u1` is not declared as mutable

    let mut u2 = User {
        id: 1002,
        name: String::from("liu"),
        pwd: String::from("!@#$"),
        desc: String::from("bad boy"),
    };
    println!("u2 = {:?}", u2);
    u2.name.push_str("...");
    println!("u2 = {:?}", u2);
    let mut u2_name = u2.name;
    println!("u2_name = {:?}", u2_name);
    u2_name.push('😊');
    println!("u2_name = {:?}", u2_name);
    // println!("u2 = {:?}", u2); // borrow of partially moved value: `u2`
    u2.name = u2_name;
    println!("u2 = {:?}", u2);

    let u3_pwd = String::from("!QAZ");
    let u3_name = String::from("liu003");
    let u3 = build_user(&u3_pwd, &u3_name);
    println!("u3 = {:?}", u3);
    println!("u3_pwd = {:?}, u3_name = {:?}", u3_pwd, u3_name);

    let p1 = Point(66, 88);
    println!("p1 = {:?}", p1);
    println!("p1.0 = {:?}, p1.1 = {:?}", p1.0, p1.1);
    // p1.0 += 100; // cannot assign to `p1.0`, as `p1` is not declared as mutable

    let mut p2 = Point(11, 22);
    println!("p2 = {:?}", p2);
    p2.0 += 100;
    println!("p2 = {:?}", p2);

    let f = Fuck();
    println!("f = {:?}", f);
    println!("f = {:#?}", f);
}

fn build_user(pwd: &String, name: &String) -> User {
    let n = rand::thread_rng().gen_range(0..100);
    User {
        id: 1000 + n,
        pwd: pwd.clone(),
        name: name.clone(),
        desc: String::from(""),
    }
}
