/*
智能指针需要实现 Deref 与 Drop 两个 trait

引用与解引用
    1) 对于复合类型(如struct)的(可变)引用不可解引用
    2) 对于复合类型的未实现Copy trait的成员的(可变)引用也不可以解引用
    3) 对于1、2两点,可以连续(可变)引用
*/

fn main() {
    println!("*** *** Smart Box *** ***");
    {
        println!("*** *** *** ***");
        let a = Box::new(5);
        let b = Box::new(String::from("abc"));
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        // println!("a == 5 : {:?}", a == 5); // mismatched types
        println!("*a == 5 : {:?}", *a == 5);
        println!(
            "*b == String::from(\"abc\") : {:?}",
            *b == String::from("abc")
        );
    }

    {
        println!("*** *** *** ***");
        /*
            pub struct String {
                vec: Vec<u8>,
            }
        */
        let a = String::from("xyz");
        /*
            pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
                buf: RawVec<T, A>,
                len: usize,
            }
            pub(crate) struct RawVec<T, A: Allocator = Global> {
                ptr: Unique<T>,
                cap: usize,
                alloc: A,
            }
        */
        let b = vec![11, 22, 33];
        println!("a = {:?}", a);
        println!("b = {:?}", b);
    }

    {
        println!("*** *** *** ***");
        let user_box = Box::new(User {
            id: 1001,
            name: String::from("user-1001"),
        });
        println!("user_box = {:?}", user_box);
        println!("user_box = {:?}", &user_box);
    }

    {
        println!("*** *** *** ***");
        let n0 = Node::None;
        let n1 = Node::Value(String::from("aaa"), &n0);
        let n2 = Node::Value(String::from("bbb"), &n1);
        let n3 = Node::Value(String::from("ccc"), &n2);
        println!("n3 = {:?}", n3);
    }

    {
        println!("*** *** *** ***");
        let list = Iterm::Value(
            String::from("cccc"),
            Box::new(Iterm::Value(
                String::from("bbbb"),
                Box::new(Iterm::Value(String::from("aaaa"), Box::new(Iterm::None))),
            )),
        );
        println!("list = {:?}", list);
    }

    {
        println!("*** *** *** ***");
        let mut a: &dyn std::fmt::Debug = &Node::Value(String::from("Node--xyz"), &Node::None);
        println!("a = {:?}", a);
        let iterm_list = Iterm::Value(String::from("...Iterm..."), Box::new(Iterm::None));
        a = &iterm_list;
        println!("a = {:?}", a);
    }

    {
        println!("*** *** *** ***");
        let a = Node::Value(String::from("Node--xyz"), &Node::None);
        let b = Iterm::Value(String::from("...Iterm..."), Box::new(Iterm::None));
        demo(&a);
        demo(&b)
    }

    {
        println!("*** *** *** ***");
        let a = Node::Value(String::from("Node--xyz"), &Node::None);
        let b = Iterm::Value(String::from("...Iterm..."), Box::new(Iterm::None));
        demo2(Box::new(a));
        demo2(Box::new(b));
    }

    {
        println!("*** *** *** ***");
        let a = &5;
        // println!("a == 5 : {:?}", a == 5); // can't compare `&{integer}` with `{integer}`
        println!("a == &5 : {:?}", a == &5);
        println!("*a == 5 : {:?}", *a == 5);

        let b = Box::new(5);
        let c = Box::new(&5);
        let d = Box::new(*a);
        let e = Box::new(a);
        // println!("b == 5 : {:?}", b == 5); // mismatched types
        println!("*b == 5 : {:?}", *b == 5);
        // println!("*c == 5 : {:?}", *c == 5); // can't compare `&{integer}` with `{integer}`
        println!("*c == 5 : {:?}", **c == 5);
        println!("b == d : {:?}", b == d);
        println!("c == e : {:?}", c == e);

        let a = 5;
        let mut b = 10;
        b = b / 2;
        let pa = &a;
        let pb = &b;
        println!("pa == pb : {:?}", pa == pb);

        let sb = Box::new(String::from("abc"));
        println!("sb = {:?}", sb);
        let sbv = *sb;
        println!("sbv = {:?}", sbv);
    }

    {
        println!("*** *** *** ***");

        let b1 = MyBox(66);
        println!("--- ---");
        // *(b1.deref()); // no method named `deref` found for struct `MyBox<{integer}>` in the current scope
        let v1 = *b1;
        println!("--- ---");
        println!("b1 = {:?}, v1 = {:?}", b1, v1);

        let b2 = MyBox(String::from("xyz"));
        // let v2 = *b2; // cannot move out of dereference of `MyBox<String>`
        fn hello(s: &str) {
            println!("===> hello: s={:?}", s);
        }
        hello(&b2);
        hello(&(*b2)[..]);
    }

    {
        println!("*** *** *** ***");
        let u1 = Box::new(User {
            id: 1001,
            name: String::from("u1001"),
        });
        let u2 = Box::new(User {
            id: 1002,
            name: String::from("u1002"),
        });
        let u3 = User {
            id: 1003,
            name: String::from("u1003"),
        };
        let u4 = User {
            id: 1004,
            name: String::from("u1004"),
        };
        println!("--- ---");
        // u1.drop(); // explicit use of destructor method
        // u3.drop(); // explicit use of destructor method
        drop(u1);
        drop(u3);
        println!("=== ===");

        // println!(
        //     "u1.name = {:?}, u2.name = {:?}, u3.name = {:?}, u4.name = {:?}",
        //     u1.name, u2.name, u3.name, u4.name
        // );
        // // borrow of moved value: `u1`
        // // borrow of moved value: `u3`

        println!("u2.name = {:?}, u4.name = {:?}", u2.name, u4.name);
    }
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox drop called");
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let r = &self.0;
        println!("MyBox deref call >>>");
        println!("MyBox deref call <<<");
        r
    }
}

pub fn demo2(v: Box<dyn std::fmt::Debug>) {
    println!("demo2 ===>>> v : {:?}", v);
}

pub fn demo(v: &dyn std::fmt::Debug) {
    println!("demo ===>>> v : {:?}", v);
}

#[derive(Debug)]
enum Iterm {
    Value(String, Box<Iterm>),
    None,
}

#[derive(Debug)]
enum Node<'a> {
    Value(String, &'a Node<'a>),
    None,
}

#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
}

impl Drop for User {
    fn drop(&mut self) {
        println!("User drop: id={:?}, name={:?}", self.id, self.name);
    }
}
