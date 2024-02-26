fn main() {
    println!("*** *** Smart Leak *** ***");

    {
        println!("*** *** *** ***");

        let a = std::rc::Rc::new(Iterm::Value(
            11,
            std::cell::RefCell::new(std::rc::Rc::new(Iterm::Nil)),
        ));
        println!("a = {:?}", a);

        let b = std::rc::Rc::new(Iterm::Value(
            22,
            std::cell::RefCell::new(std::rc::Rc::clone(&a)),
        ));
        println!("b = {:?}", b);

        if let Option::Some(v) = a.next() {
            *v.borrow_mut() = std::rc::Rc::clone(&b);
        }

        // println!("a = {:?}", a); // fatal runtime error: stack overflow
        // println!("b = {:?}", b); // fatal runtime error: stack overflow
    }

    {
        println!("*** *** *** ***");
        let node1 = std::rc::Rc::new(Node {
            value: 1001,
            parent: std::cell::RefCell::new(std::rc::Weak::new()),
            children: std::cell::RefCell::new(vec![]),
        });
        println!("node1 = {:?}", node1);

        let node2 = std::rc::Rc::new(Node {
            value: 1002,
            parent: std::cell::RefCell::new(std::rc::Weak::new()),
            children: std::cell::RefCell::new(vec![]),
        });
        println!("node2 = {:?}", node2);

        let node3 = std::rc::Rc::new(Node {
            value: 1003,
            parent: std::cell::RefCell::new(std::rc::Weak::new()),
            children: std::cell::RefCell::new(vec![]),
        });
        println!("node3 = {:?}", node3);

        let root = std::rc::Rc::new(Node {
            value: 100,
            parent: std::cell::RefCell::new(std::rc::Weak::new()),
            children: std::cell::RefCell::new(vec![
                std::rc::Rc::clone(&node1),
                std::rc::Rc::clone(&node2),
                std::rc::Rc::clone(&node3),
            ]),
        });
        println!("root = {:?}", root);
        println!("node1.parent = {:?}", node1.parent.borrow().upgrade());
        println!(
            "node1: strong_count={:?}, weak_count={:?}",
            std::rc::Rc::strong_count(&node1),
            std::rc::Rc::weak_count(&node1)
        );
        println!(
            "root: strong_count={:?}, weak_count={:?}",
            std::rc::Rc::strong_count(&root),
            std::rc::Rc::weak_count(&root)
        );

        println!("--------------------");

        *node1.parent.borrow_mut() = std::rc::Rc::downgrade(&root);
        *node2.parent.borrow_mut() = std::rc::Rc::downgrade(&root);
        *node3.parent.borrow_mut() = std::rc::Rc::downgrade(&root);

        println!("root = {:?}", root);
        println!("node1.parent = {:?}", node1.parent.borrow().upgrade());

        println!(
            "node1: strong_count={:?}, weak_count={:?}",
            std::rc::Rc::strong_count(&node1),
            std::rc::Rc::weak_count(&node1)
        );
        println!(
            "root: strong_count={:?}, weak_count={:?}",
            std::rc::Rc::strong_count(&root),
            std::rc::Rc::weak_count(&root)
        );
    }
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: std::cell::RefCell<std::rc::Weak<Node>>,
    children: std::cell::RefCell<Vec<std::rc::Rc<Node>>>,
}

#[derive(Debug)]
enum Iterm {
    Value(i32, std::cell::RefCell<std::rc::Rc<Iterm>>),
    Nil,
}

impl Iterm {
    pub fn next(&self) -> Option<&std::cell::RefCell<std::rc::Rc<Iterm>>> {
        match self {
            Iterm::Value(_, iterm) => Option::Some(iterm),
            Iterm::Nil => Option::None,
        }
    }
}
