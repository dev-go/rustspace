/*
生命周期的省略规则:
    1. 每个引用类型的参数都有自己的生命周期
    2. 如果只有1个输入生命周期参数, 那么该生命周期被赋给所有的输出生命周期参数
    3. 如果有多个输入生命周期参数, 但其中一个事 &self 或 &mut self (即: 方法), 那么self的生命周期会被赋给所有的输出生命周期参数
*/

fn main() {
    println!("Hello, Lifetime!");

    {
        println!("=== >>>");
        let x = String::from("x");
        let r: &str;
        {
            let y = String::from("y");
            r = mirror(&x, &y);
        }
        println!("x = {:?}, r = {:?}", x, r);
    }

    // {
    //     println!("=== >>>");
    //     let x: String = String::from("x");
    //     let r: &str;
    //     {
    //         let y = String::from("y");
    //         r = mirror(&y, &x); // `y` does not live long enough
    //     }
    //     println!("x = {:?}, r = {:?}", x, r);
    // }

    {
        println!("=== >>>");
        let x = String::from("x");
        let r: &str;
        {
            let y = "y"; // let y: &'static str = "y";
            r = mirror(y, &x);
        }
        println!("x = {:?}, r = {:?}", x, r);
    }

    // {
    //     println!("=== >>>");
    //     let x = String::from("x");
    //     let r: &str;
    //     {
    //         let y = String::from("yyy");
    //         r = longest(&x, &y); // `y` does not live long enough
    //     }
    //     println!("x = {:?}, r = {:?}", x, r);
    // }

    {
        println!("=== >>>");
        let x = String::from("x");
        let r: &str;
        {
            let y = "yyy";
            r = longest(&x, &y);
        }
        println!("x = {:?}, r = {:?}", x, r);
    }

    {
        println!("=== >>>");
        let x = String::from("x");
        let r: &str;
        {
            let y = String::from("yyy");
            r = longest(&x, &y);
            println!("x = {:?}, r = {:?}", x, r);
        }
        println!("x = {:?}", x);
    }

    // {
    //     println!("=== >>>");
    //     let wallet: Wallet;
    //     let user1 = User {
    //         name: String::from("user1"),
    //         wallet: &wallet, // used binding `wallet` is possibly-uninitialized
    //     };
    //     wallet = Wallet { coin: 888 }; // `wallet` is assigned to here but it was already borrowed
    //     println!("user1 = {:?}", user1);
    // }

    {
        println!("=== >>>");
        let wallet_desc = String::from("gold");
        let wallet = Wallet {
            desc: &wallet_desc,
            coin: 888,
        };
        let user1 = User {
            name: String::from("user1"),
            wallet: &wallet,
        };
        {
            let user2 = User {
                name: String::from("user2"),
                wallet: &wallet,
            };
            println!("user1 = {:?}, user2 = {:?}", user1, user2);
        }
        println!("user1 = {:?}", user1);
    }

    {
        println!("=== >>>");
        let x = String::from("xxx");
        let p: Point;
        let x2: &str;
        let y2: &str;
        {
            let y = String::from("yyy");
            p = Point { x: &x, y: &y };
            println!("p = {:?}", p);
            x2 = p.x;
            y2 = p.y;
            println!("x2 = {:?}", x2);
            println!("y2 = {:?}", y2);
        }
        // println!("p = {:?}", p); // `y` does not live long enough
        println!("x2 = {:?}", x2);
        // println!("y2 = {:?}", y2); // `y` does not live long enough
    }
}

// fn mirror(x: &str, y: &str) -> &str { // missing lifetime specifier
//     println!("[mirror] x = {}, y = {}", x, y);
//     return x;
// }

fn mirror<'a>(x: &'a str, y: &str) -> &'a str {
    println!("[mirror] x = {}, y = {}", x, y);
    return x;
}

// fn longest(x: &str, y: &str) -> &str { // missing lifetime specifier
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest<'a>(x: &'a str, y: &str) -> &'a str { // explicit lifetime required in the type of `y`
//     if x.len() > y.len() {
//         x
//     } else {
//         y // explicit lifetime required in the type of `y`
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct User<'a> {
    name: String,
    wallet: &'a Wallet<'a>,
}

#[derive(Debug)]
struct Wallet<'a> {
    desc: &'a String,
    coin: u32,
}

#[derive(Debug)]
struct Point<'a, 'b> {
    x: &'a str,
    y: &'b str,
}
