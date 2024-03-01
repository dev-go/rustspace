fn main() {
    println!("*** *** Advance Trait *** ***");

    {
        println!("*** *** *** ***");
        trait Iter1 {
            type Iterm;
            fn next(&mut self) -> Option<Self::Iterm>;
        }

        trait Iter2<T> {
            fn next(&mut self) -> Option<T>;
        }

        struct Range1 {
            max: u32,
            value: u32,
        }

        impl Iter1 for Range1 {
            type Iterm = String;
            fn next(&mut self) -> Option<Self::Iterm> {
                let new_value = self.value + 1;
                if new_value > self.max {
                    Option::None
                } else {
                    self.value = new_value;
                    Option::Some(self.value.to_string())
                }
            }
        }

        // impl Iter1 for Range1 { // conflicting implementations of trait `Iter1` for type `Range1`
        //     type Iterm = u32;

        //     fn next(&mut self) -> Option<Self::Iterm> {
        //         let new_value = self.value;
        //         if new_value > self.max {
        //             Option::None
        //         } else {
        //             self.value = new_value;
        //             Option::Some(self.value)
        //         }
        //     }
        // }

        struct Range2 {
            max: u32,
            value: u32,
        }

        // impl<T> Iter2<T> for Range2 {
        //     fn next(&mut self) -> Option<T> {
        //         Option::None
        //     }
        // }

        impl Iter2<u32> for Range2 {
            fn next(&mut self) -> Option<u32> {
                let new_value = self.value + 1;
                if new_value > self.max {
                    Option::None
                } else {
                    self.value = new_value;
                    Option::Some(self.value)
                }
            }
        }

        impl Iter2<String> for Range2 {
            fn next(&mut self) -> Option<String> {
                let new_value = self.value + 1;
                if new_value > self.max {
                    Option::None
                } else {
                    self.value = new_value;
                    Option::Some(self.value.to_string())
                }
            }
        }
    }

    {
        println!("*** *** *** ***");

        #[derive(Debug)]
        struct Point {
            x: u32,
            y: u32,
        }

        impl std::ops::Add for Point {
            type Output = Point;

            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        let p1 = Point { x: 11, y: 22 };
        let p2 = Point { x: 100, y: 200 };
        let p3 = p1 + p2;
        println!("p3 = {:?}", p3);
    }

    {
        println!("*** *** *** ***");

        #[derive(Debug)]
        struct Meter(u32);

        #[derive(Debug)]
        struct KMeter(u32);

        // pub trait Add<Rhs = Self> {

        impl std::ops::Add<KMeter> for Meter {
            type Output = Meter;

            fn add(self, rhs: KMeter) -> Self::Output {
                Meter(self.0 + rhs.0 * 1000)
            }
        }

        let m = Meter(111);
        let km = KMeter(2);
        let all = m + km;
        println!("all = {:?}", all);
    }

    {
        println!("*** *** *** ***");

        trait Polit {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        #[derive(Debug)]
        struct User {
            name: String,
        }

        impl User {
            pub fn fly(&self) {
                println!("User fly: name = {:?}", self.name);
            }
        }

        impl Polit for User {
            fn fly(&self) {
                println!("Polit User fly");
            }
        }

        impl Wizard for User {
            fn fly(&self) {
                println!("Wizard User fly");
            }
        }

        let u = User {
            name: String::from("liu"),
        };
        u.fly();
        (&u as &dyn Polit).fly();
        (&u as &dyn Wizard).fly();
        // (u as dyn Wizard).fly(); // cast to unsized type: `User` as `dyn Wizard`
        // (u as Wizard).fly(); // cast to unsized type: `User` as `dyn Wizard`

        Polit::fly(&u);
        Wizard::fly(&u);
    }

    {
        println!("*** *** *** ***");

        trait Animal {
            fn name() -> String;
        }

        struct Dog();

        impl Dog {
            fn name() -> String {
                String::from("dog")
            }
        }

        impl Animal for Dog {
            fn name() -> String {
                String::from("dog - animal")
            }
        }

        println!("dog name: {:?}", Dog::name());
        println!("dog <animal> name: {:?}", <Dog as Animal>::name());
    }

    {
        println!("*** *** *** ***");

        trait OutlinePrint: std::fmt::Display {
            fn outline_print(&self) {
                let len = self.to_string().len();
                println!("{}", "*".repeat(len + 8));
                println!("**  {}  **", self);
                println!("{}", "*".repeat(len + 8));
            }
        }

        {
            #[derive(Debug)]
            struct Point {
                x: u32,
                y: u32,
            }

            // impl OutlinePrint for Point {} // `main::Point` doesn't implement `std::fmt::Display`
        }

        {
            #[derive(Debug)]
            struct Point {
                x: u32,
                y: u32,
            }

            impl OutlinePrint for Point {}

            impl std::fmt::Display for Point {
                fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    // std::fmt::write(fmt, format_args!("Point{{x: {}, y: {}}}", self.x, self.y))
                    write!(fmt, "Point{{x: {}, y: {}}}", self.x, self.y)
                }
            }
        }
    }

    {
        println!("*** *** *** ***");

        #[derive(Debug)]
        struct VecWrapper(Vec<String>);

        impl std::fmt::Display for VecWrapper {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "Vec[{}]", self.0.join(", "))
            }
        }

        let w = VecWrapper(vec![
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ]);
        println!("w = {}", w);
        println!("w = {:?}", w);
    }
}
