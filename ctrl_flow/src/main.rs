fn main() {
    let mut score = 58;
    let (mut level, mut ok) = score_level(score);
    println!("score = {score}, level = {level}, ok = {ok}");

    score += 10;
    (level, ok) = score_level(score);
    println!("score = {score}, level = {level}, ok = {ok}");

    score += 19;
    (level, ok) = score_level(score);
    println!("score = {score}, level = {level}, ok = {ok}");

    score += 50;
    (level, ok) = score_level(score);
    println!("score = {score}, level = {level}, ok = {ok}");

    let c = if ok { '😊' } else { '😭' };
    println!("c = {:?}", c);

    // let d = if ok { "OK" } else { -1 }; // error[E0308]: `if` and `else` have incompatible types
    // println!("d = {:?}", d);

    loop_demo();
    for_demo();
    while_demo();
}

fn score_level(n: u8) -> (String, bool) {
    if n > 100 {
        (String::from(""), false)
    } else if n >= 80 {
        (String::from("优秀"), true)
    } else if n >= 60 {
        (String::from("及格"), true)
    } else {
        (String::from("不及格"), true)
    }
}

fn loop_demo() {
    let mut counter = 0;
    let i = loop {
        counter += 2;
        if counter > 100 {
            break counter + 100;
        }
    };
    println!("counter = {counter}, i = {i}");
}

fn for_demo() {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("1 + 2 + 3 + ... + 100 = {sum}");

    let mut counter = 0;
    let end = for i in 1..=100 {
        counter += 2;
        if counter > 100 {
            // break i; // error[E0571]: `break` with value from a `for` loop
            break;
        }
    };
    println!("counter = {counter}");
}

fn while_demo() {
    let a = [11, 22, 33, 44];
    let mut i = 0;
    while i < a.len() {
        let e = a[i];
        println!("a[{i}] = {e}");
        i += 1;
    }
}
