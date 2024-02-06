use std::io::Read;

fn main() {
    println!("hello, error!");

    {
        println!("=== >>> panic");

        // panic!("!!! panic in main !!!");

        f1();
    }

    {
        println!("=== >>> result");

        let r1: Result<u8, String> = Result::Ok(88);
        let r2: Result<u8, String> = Result::Err(String::from("not found"));
        r1.expect("r1 failed");
        // let v1 = r1.unwrap(); // use of moved value: `r1`

        // r2.expect("r2 failed");
        // let v2 = r2.unwrap();
    }

    {
        println!("=== >>> ?");

        let opt = opt_demo();
        println!("opt = {:?}", opt);

        let rslt = rslt_demo();
        println!("rslt = {:?}", rslt);
    }

    {
        println!("=== >>> demo");
        let file = "hello.txt";
        let content = read_file(file);
        println!("file: {:?}, content: {:?}", file, content);
    }
}

fn read_file(file: &str) -> String {
    let mut f = std::fs::File::open(file).unwrap_or_else(|err| match err.kind() {
        std::io::ErrorKind::NotFound => std::fs::File::create("hello.txt").unwrap_or_else(|err| {
            panic!("create file failed: {:?}", err);
        }),
        oe => {
            panic!("open file failed: {:?}", oe);
        }
    });
    let mut buf = [0_u8; 128];
    let result = f.read(&mut buf);
    match result {
        Ok(n) => {
            if n <= 0 {
                return String::from("");
            }
            let content_result = String::from_utf8(buf[..n].to_vec());
            match content_result {
                Ok(content) => {
                    println!("read file: ok, content={:?}", content);
                    return content;
                }
                Err(err) => {
                    panic!("convert utf8 failed: {:?}", err);
                }
            };
        }
        Err(err) => {
            panic!("read content failed: {:?}", err);
        }
    };
}

fn opt_demo() -> Option<u8> {
    let mut r1 = Option::Some(66);
    r1?;
    println!(">>> === opt_demo 1");
    r1 = Option::None;
    r1?;
    println!(">>> === opt_demo 2");
    return r1;
}

fn rslt_demo() -> Result<u8, String> {
    let mut r1 = Result::Ok(66);
    r1?;
    println!(">>> === rslt_demo 1");
    r1 = Result::Err(String::from("time out"));
    // r1?;
    // return r1; // error[E0382]: use of moved value: `r1`
    r1.clone()?;
    return r1;
}

fn f1() {
    println!("=> f1");
    let r = std::panic::catch_unwind(|| {
        f2();
    });

    // std::panic::resume_unwind(payload)

    println!("catch: r = {:#?}", r);
    if let Result::Err(err) = r {
        println!("catch err: {:#?}", err);
    }
    println!("<= f1");
}

fn f2() {
    println!("==>> f2");
    f3();
    println!("<<== f2");
}

fn f3() {
    println!("===>>> f3");
    panic!("!!! panic in f3 !!!");
    println!("<<<=== f3");
}
