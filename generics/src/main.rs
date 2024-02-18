fn main() {
    {
        let mut list1 = [11, 55, 33, 66, 22];
        println!("list1 = {:?}", list1);
        let max11 = largest(&list1);
        println!("max11 = {:?}", max11);
        let max12 = largest(&list1[..]);
        println!("max12 = {:?}", max12);
        let max13 = largest(&list1[..0]);
        println!("max13 = {:?}", max13);

        // list1[0] = 99; // cannot assign to `list1[_]` because it is borrowed
        // println!("list1 = {:?}", list1);
        // println!("max11 = {:?}", max11); // borrow later used here

        list1[0] = 88;
        println!("list1 = {:?}", list1);
    }

    {
        let mut list2 = [
            String::from("a"),
            String::from("c"),
            String::from("b"),
            String::from("e"),
            String::from("d"),
        ];
        println!("list2 = {:?}", list2);

        let max21 = largest(&list2[..]);
        println!("max21 = {:?}", max21);

        // list2[0] = String::from("xyz"); // cannot assign to `list2[_]` because it is borrowed
        // println!("list2 = {:?}", list2);
        // println!("max21 = {:?}", max21);

        list2[0] = String::from("opq");
        println!("list2 = {:?}", list2);
    }

    {
        let p1 = Point { x: 3.1415, y: 8 };
        println!("p1 = {:?}", p1);
        let p2 = Point {
            x: true,
            y: String::from("xyz"),
        };
        println!("p2 = {:?}", p2);
        let p3 = p1.mix_up(p2);
        println!("p3 = {:?}", p3);
        // println!("p1 = {:?}", p1); // borrow of moved value: `p1`
        // println!("p2 = {:?}", p2); // borrow of moved value: `p2`
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
    if list.len() <= 0 {
        return Option::None;
    }
    let mut max = &list[0];
    for e in list {
        if e > max {
            max = e;
        }
    }
    return Option::Some(max);
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    pub fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
