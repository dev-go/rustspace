use rand::Rng;

fn main() {
    println!("*** Guess Number ***");
    let secret = rand::thread_rng().gen_range(0..100);
    loop {
        println!("Please input a number:");
        let mut input = String::from("");
        std::io::stdin()
            .read_line(&mut input)
            .expect("read line from stdin failed");
        let input: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("parse your input to a number failed: {e}");
                continue;
            }
        };
        match input.cmp(&secret) {
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("### secret = {secret}");
}
