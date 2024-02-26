use std::cell::RefMut;

fn main() {
    println!("*** *** Smart RefCell *** ***");

    {
        println!("*** *** *** ***");
        let a = String::from("aaa");
        // let b = &mut a; // cannot borrow `a` as mutable, as it is not declared as mutable

        let c = std::cell::RefCell::new(a);
        println!("c = {:?}", c);
        c.borrow_mut().push_str("...");
        println!("c = {:?}", c);

        // println!("a = {:?}", a); // borrow of moved value: `a`
    }

    {
        println!("*** *** *** ***");
        let a = String::from("aaa");
        let b = std::cell::RefCell::new(&a);
        let c = std::cell::RefCell::new(&a);
        // b.borrow_mut().push_str("..."); // cannot borrow data in dereference of `RefMut<'_, &String>` as mutable
        let d = c.borrow_mut();
        // d.push_str("xxx"); // cannot borrow data in dereference of `RefMut<'_, &String>` as mutable
    }

    {
        println!("*** *** *** ***");

        // pub struct Sender1 {
        //     data: Vec<String>,
        // }

        // impl ISender for Sender1 {
        //     // fn send(&self, msg: &str) {
        //     //     self.data.push(String::from(msg)); // cannot borrow `self.data` as mutable, as it is behind a `&` reference
        //     // }

        //     fn send(&mut self, msg: &str) { // method `send` has an incompatible type for trait
        //         self.data.push(String::from(msg));
        //     }
        // }

        #[derive(Debug)]
        pub struct Sender2 {
            data: std::cell::RefCell<Vec<String>>,
        }

        impl ISender for Sender2 {
            fn send(&self, msg: &str) {
                self.data.borrow_mut().push(String::from(msg));
            }
        }

        let sender2 = Sender2 {
            data: std::cell::RefCell::new(Vec::new()),
        };
        println!("sender2 = {:?}", sender2);
        let mut alerter = Alerter::new(&sender2, 100);
        alerter.set_value(50);
        println!("sender2 = {:?}", sender2);
        alerter.set_value(80);
        println!("sender2 = {:?}", sender2);
        alerter.set_value(95);
        println!("sender2 = {:?}", sender2);
    }

    {
        println!("*** *** *** ***");
        let e1 = std::rc::Rc::new(std::cell::RefCell::new(11));
        let e2 = std::rc::Rc::new(std::cell::RefCell::new(22));
        let e31 = std::rc::Rc::new(std::cell::RefCell::new(331));
        let e32 = std::rc::Rc::new(std::cell::RefCell::new(332));

        let a = std::rc::Rc::new(Iterm::Value(
            std::rc::Rc::clone(&e2),
            std::rc::Rc::new(Iterm::Value(
                std::rc::Rc::clone(&e1),
                std::rc::Rc::new(Iterm::Nil),
            )),
        ));
        println!("a = {:?}", a);
        let b = Iterm::Value(std::rc::Rc::clone(&e31), std::rc::Rc::clone(&a));
        println!("b = {:?}", b);
        let c = Iterm::Value(std::rc::Rc::clone(&e32), std::rc::Rc::clone(&a));
        println!("c = {:?}", c);
        *e2.borrow_mut() += 10000;
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("c = {:?}", c);
    }
}

#[derive(Debug)]
enum Iterm {
    Value(std::rc::Rc<std::cell::RefCell<u32>>, std::rc::Rc<Iterm>),
    Nil,
}

pub trait ISender {
    fn send(&self, msg: &str);
}

pub struct Alerter<'a, T: ISender> {
    sender: &'a T,
    max: u32,
    value: u32,
}

impl<'a, T: ISender> Alerter<'a, T> {
    pub fn new(sender: &'a T, max: u32) -> Alerter<'a, T> {
        Alerter {
            sender: sender,
            max: max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: u32) {
        println!(
            "Alert.set_value: max={:?}, old_value={:?}, new_value={:?}",
            self.max, self.value, value
        );
        self.value = value;
        let p = self.value as f64 / self.max as f64;
        if p >= 1.0 {
            self.sender.send("Error: over max");
        } else if p >= 0.9 {
            self.sender.send("Urgent: over 90%");
        } else if p >= 0.75 {
            self.sender.send("Warn: over 75%");
        }
    }
}
