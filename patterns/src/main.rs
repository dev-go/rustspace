fn main() {
    println!("*** *** Patterns *** ***");

    {
        println!("=== === if let === ===");
        let color: Option<&str> = Option::None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
        if let Option::Some(v) = color {
            println!("using color: {:?}", color);
        } else if is_tuesday {
            println!("today is tuesday, using color: green");
        } else if let Result::Ok(v) = age {
            if v >= 18 {
                println!("using color: yellow");
            } else {
                println!("using color: orange");
            }
        } else {
            println!("using color: blue");
        }
    }

    {
        println!("=== === while let === ===");
        let mut stack = Vec::<String>::new();
        stack.push(String::from("aaa"));
        stack.push(String::from("bbb"));
        stack.push(String::from("ccc"));
        println!("stack = {:?}", stack);

        while let Option::Some(v) = stack.pop() {
            println!("stack pop: {:?}", v);
        }
        println!("stack = {:?}", stack);
    }

    {
        println!("=== === for === ===");
        let mut list: Vec<String> = Vec::<String>::new();
        list.push(String::from("aaa"));
        list.push(String::from("bbb"));
        list.push(String::from("ccc"));
        println!("list = {:?}", list);

        for (idx, elem) in list.iter().enumerate() {
            println!("list[{idx}] = {elem}");
        }
        println!("list = {:?}", list);
    }

    {
        println!("=== === let === ===");
        let a = 1024;
        let (b, c, d) = (String::from("abc"), 66, 3.14);
        // let (e, f) = (true, 3.14, 55); // mismatched types
        println!("a = {a}, b = {b}, c = {c}, d = {d}");
    }

    {
        println!("=== === func parameter === ===");
        fn foo(x: i32) {
            println!("foo: x = {x}");
        }
        fn bar(&(x, y): &(i32, i32)) {
            println!("bar: x = {x}, y = {y}");
        }
        let a = 100;
        foo(a);
        let p = (66, 88);
        bar(&p);
    }

    {
        println!("=== === refutable === ===");
        let a = Option::Some(5);
        println!("a = {:?}", a);
        // let Option::Some(b) = a; // refutable pattern in local binding
        // let Option::Some(c) = Option::Some(5); // refutable pattern in local binding
        let Option::Some(d) = a else { todo!() };
        println!("d = {d}");
        if let Option::Some(e) = a {
            println!("e = {e}");
        }
    }

    {
        println!("=== === match === ===");
        // let x = 2;
        let x = 5;
        match x {
            1 => println!("x: one"),
            2 => println!("x: two"),
            3 => println!("x: three"),
            _ => println!("x: default arm ({x})"),
        }
    }

    {
        println!("=== === match variable === ===");
        let a = Option::Some(5);
        let b = 10;
        match a {
            Some(4) => println!("1>>> a is 4"),
            Some(b) => println!("2>>> a is {b}"),
            _ => println!("3>>> default: a is {:?}", a),
        }
        println!("a = {:?}, b = {:?}", a, b);
    }

    {
        println!("=== === match | === ===");
        let x = 1;
        match x {
            1 | 2 => println!("one or two: {x}"),
            3 => println!("three: {x}"),
            _ => println!("other value: {x}"),
        }
    }

    {
        println!("=== === match range === ===");
        let a = 3;
        match a {
            1..=5 => println!("one, two, three, four, five: {a}"),
            _ => println!("other value: {a}"),
        }

        // let b = 'a';
        let b = 'A';
        match b {
            'a'..='z' => println!("a-z, b = {:?}", b),
            'A'..='Z' => println!("A-Z, b = {:?}", b),
            _ => println!("other value, b = {:?}", b),
        }
    }

    {
        println!("=== === match struct === ===");
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 66, y: 88 };
        let Point { x: a, y: b } = p;
        println!("a = {a}, b = {b}");

        let Point { x, y } = p;
        println!("x = {x}, y = {y}");

        // let p = Point { x: 0, y: 0 };
        // let p = Point { x: 0, y: 88 };
        // let p = Point { x: 66, y: 0 };
        match p {
            Point { x: 0, y: 0 } => println!("Point.x = 0 and Point.y = 0, {:?}", p),
            Point { x: 0, y } => println!("Point.x = 0, y = {y}, {:?}", p),
            Point { x, y: 0 } => println!("Point.y = 0, x = {x}, {:?}", p),
            Point { x, y } => println!("x = {x}, y = {y}, Point: {:?}", p),
        }
    }

    {
        println!("=== === match enum === ===");
        enum Msg {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8),
        }
        let msg = Msg::ChangeColor(11, 22, 33);
        match msg {
            Msg::Quit => println!("Msg::Quit"),
            Msg::Move { x, y } => println!("Msg::Move  x={x}, y={y}"),
            Msg::Write(s) => println!("Msg::Write  s={s}"),
            Msg::ChangeColor(r, g, b) => println!("Msg::ChangeColor  r={r}, g={g}, b={b}"),
        }
    }

    {
        println!("=== === match === ===");

        enum Color {
            Rgb(u8, u8, u8),
            Hsv(u8, u8, u8),
        }

        enum Msg {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Msg::ChangeColor(Color::Hsv(100, 110, 120));
        match msg {
            Msg::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Msg::ChangeColor -> Color::Hsv : h={h}, s={s}, v={v}")
            }
            _ => (),
        }
    }

    {
        println!("=== === match struct/tuple === ===");
        struct Point {
            x: i32,
            y: i32,
        }
        let ((a, b), Point { x: c, y: d }) = ((111, "xyz"), Point { x: 1024, y: 2048 });
        println!("a = {a}, b = {b}, c = {c}, d = {d}");
    }

    {
        println!("=== === match ignore _ === ===");
        fn foo(_: i32, b: String) {
            println!("foo: b = {:?}", b);
        }
        foo(111, String::from("abc"));

        let mut setting = Option::Some(5);
        let new_setting = Option::Some(10);
        match (setting, new_setting) {
            (Option::Some(_), Option::Some(_)) => {
                println!("can not overwrite an existing settings");
            }
            _ => setting = new_setting,
        }
        println!("setting = {:?}", setting);

        let a = (11, 22, 33, 44, 55);
        match a {
            (e0, _, _, e3, _) => println!("e0 = {:?}, e3 = {:?}", e0, e3),
        }

        let x = 55; // unused variable: `x`
        let _y = 66;

        let p = Option::Some(String::from("abc"));
        let q = Option::Some(String::from("xyz"));
        println!("p = {:?}, q = {:?}", p, q);
        if let Option::Some(_v) = p {
            println!("found a string in p");
        }
        if let Option::Some(_) = q {
            println!("found a string in q");
        }
        // println!("p = {:?}", p); // borrow of partially moved value: `p`
        println!("q = {:?}", q);
    }

    {
        println!("=== === match .. === ===");
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let p = Point {
            x: 11,
            y: 22,
            z: 33,
        };
        match p {
            Point { x, .. } => println!("p.x = {x}"),
        }

        let q = (11, 22, String::from("abc"), 3.14);
        match q {
            (e0, .., e3) => println!("q: e0 = {e0}, e3 = {e3}"),
        }

        // match q {
        //     (.., e2, ..) => println!("q: e2 = {e2}"), // `..` can only be used once per tuple pattern
        // }
    }

    {
        println!("=== === match arm cond === ===");
        // let a = Option::Some(3);
        // let a = Option::Some(10);
        let a: Option<i32> = Option::None;
        match a {
            Option::Some(v) if v < 5 => println!("Option::Some < 5 : v={v}, a={:?}", a),
            Option::Some(v) => println!("Option::Some >= 5 : v={v}, a = {:?}", a),
            Option::None => println!("Option::None"),
        }

        // let b = Option::Some(10);
        let b = Option::Some(100);
        // let b = Option::Some(200);
        let c = 100;
        match b {
            Option::Some(200) => println!("b is Option::Some(200)"),
            Option::Some(v) if v == c => println!("b is Option::Some(100)"),
            _ => println!("b = {:?}", b),
        }

        let d = 4;
        let e = false;
        match d {
            3 | 4 | 5 | 6 if e => println!("YES, d = {d}, e = {e}"),
            _ => println!("NO, d = {d}, e = {e}"),
        }
    }

    {
        println!("=== === match @ === ===");
        enum Msg {
            Hello { id: i32 },
        }

        let msg = Msg::Hello { id: 88 };
        // let msg = Msg::Hello { id: 166 };
        // let msg = Msg::Hello { id: 255 };
        match msg {
            Msg::Hello { id: id2 @ 0..=100 } => {
                // println!("id under 100: id = {id}"); // cannot find value `id` in this scope
                println!("id under 100: id = {id2}");
            }
            Msg::Hello { id } if id < 200 => println!(" id under 200: id = {id}"),
            Msg::Hello { id } => println!("id greater 200: id = {id}"),
        }
    }
}
