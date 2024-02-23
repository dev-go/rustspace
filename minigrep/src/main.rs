/*
$ cargo run like hello.txt
$ CASE_SENSITIVE=1 cargo run like hello.txt
*/

fn main() {
    let conf = minigrep::Config::new().unwrap_or_else(|err| {
        eprintln!("get config failed: {}", err);
        std::process::exit(1);
    });
    println!("conf = {:?}", conf);
    let content = minigrep::read(&conf).unwrap_or_else(|err| {
        eprintln!("read file failed: {}", err);
        std::process::exit(2);
    });
    println!("content = \n{}", content);
    let result = minigrep::search(&conf, &content);
    for v in result {
        println!("--- >>> {}", v);
    }
}
