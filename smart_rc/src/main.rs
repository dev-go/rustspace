fn main() {
    println!("*** Smart Rc ***");
    {
        println!("*** *** *** ***");
        let a = Iterm::Value(11, Box::new(Iterm::Nil));
        println!("a = {:?}", a);
        let b = Iterm::Value(22, Box::new(a));
        println!("b = {:?}", b);
        // let c = Iterm::Value(23, Box::new(a)); // use of moved value: `a`
        // println!("c = {:?}", c);
    }

    {
        println!("*** *** *** ***");
        let a = Elem::Value(11, std::rc::Rc::new(Elem::Nil));
        println!("a = {:?}", a);
        let b = Elem::Value(22, std::rc::Rc::new(a));
        println!("b = {:?}", b);
        // let c = Elem::Value(23, std::rc::Rc::new(a)); // use of moved value: `a`
        // println!("c = {:?}", c);
    }

    {
        println!("*** *** *** ***");
        let a = std::rc::Rc::new(Elem::Value(
            22,
            std::rc::Rc::new(Elem::Value(11, std::rc::Rc::new(Elem::Nil))),
        ));
        println!("a = {:?}", a);
        println!(
            "a.strong_count = {:?}, a.weak_count = {:?}",
            std::rc::Rc::strong_count(&a),
            std::rc::Rc::weak_count(&a)
        );

        let b = Elem::Value(22, std::rc::Rc::clone(&a));
        println!("b = {:?}", b);
        // println!(
        //     "b.strong_count = {:?}, b.weak_count = {:?}",
        //     std::rc::Rc::strong_count(&b), // mismatched types
        //     std::rc::Rc::weak_count(&b)    // mismatched types
        // );
        println!(
            "a.strong_count = {:?}, a.weak_count = {:?}",
            std::rc::Rc::strong_count(&a),
            std::rc::Rc::weak_count(&a)
        );
        {
            println!("-->");
            let c = Elem::Value(23, std::rc::Rc::clone(&a));
            println!("c = {:?}\nb = {:?}", c, b);
            println!(
                "a.strong_count = {:?}, a.weak_count = {:?}",
                std::rc::Rc::strong_count(&a),
                std::rc::Rc::weak_count(&a)
            );
            println!("<--");
        }
        println!(
            "a.strong_count = {:?}, a.weak_count = {:?}",
            std::rc::Rc::strong_count(&a),
            std::rc::Rc::weak_count(&a)
        );
    }
}

#[derive(Debug)]
enum Iterm {
    Value(u32, Box<Iterm>),
    Nil,
}

#[derive(Debug)]
enum Elem {
    Value(u32, std::rc::Rc<Elem>),
    Nil,
}
