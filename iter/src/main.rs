fn main() {
    println!("*** Iter ***");
    {
        println!("*** *** *** ***");
        let v = vec![
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        let mut iter = v.iter();
        let e1 = iter.next();
        let e2 = iter.next();
        let e3 = iter.next();
        let e4 = iter.next();
        println!("v = {:?}", v);
        println!("e1 = {:?}, e2 = {:?}, e3 = {:?}, e4 = {:?}", e1, e2, e3, e4);
    }

    {
        println!("*** *** *** ***");
        let mut v = vec![
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        let mut iter = v.iter_mut();
        let e1 = iter.next();
        e1.unwrap().push_str("...");
        let e2 = iter.next();
        let e3 = iter.next();
        let e4 = iter.next();
        // println!("e1 = {:?}, e2 = {:?}, e3 = {:?}, e4 = {:?}", e1, e2, e3, e4); // borrow of moved value: `e1`
        println!("e2 = {:?}, e3 = {:?}, e4 = {:?}", e2, e3, e4);
        println!("v = {:?}", v);
    }

    {
        println!("*** *** *** ***");
        let v = vec![
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        let mut iter = v.into_iter();
        let mut e1 = iter.next().unwrap();
        e1.push_str("...");
        let e2 = iter.next();
        let e3 = iter.next();
        let e4 = iter.next();
        // println!("e1 = {:?}, e2 = {:?}, e3 = {:?}, e4 = {:?}", e1, e2, e3, e4); // borrow of moved value: `e1`
        println!("e2 = {:?}, e3 = {:?}, e4 = {:?}", e2, e3, e4);
        // println!("v = {:?}", v); // borrow of moved value: `v`
    }

    {
        println!("*** *** *** ***");
        let v = vec![11, 22, 33];
        let iter = v.iter();
        let all: i32 = iter.sum();
        println!("all = {:?}", all);
    }

    {
        println!("*** *** *** ***");
        let v = vec![11, 22, 33];
        let v2: Vec<i32> = v.iter().map(|x| x * 10).collect();
        println!("v = {:?}", v);
        println!("v2 = {:?}", v2);
    }

    {
        println!("*** *** *** ***");
        let v = vec![
            Shoe {
                size: 37,
                model: String::from("child 37 a"),
            },
            Shoe {
                size: 38,
                model: String::from("child 38 b"),
            },
            Shoe {
                size: 37,
                model: String::from("child 37 c"),
            },
            Shoe {
                size: 38,
                model: String::from("child 38 d"),
            },
        ];
        println!("v = {:?}", v);
        let v_filter = filter_by_size(v, 37);
        println!("v_filter = {:?}", v_filter);
        // println!("v = {:?}", v); // borrow of moved value: `v`
    }

    {
        println!("*** *** *** ***");
        let cus_iter = CusIter::new(5);
        for i in cus_iter {
            println!("cusIter: {i}");
        }

        let cus_iter2: std::collections::HashMap<u32, u32> =
            CusIter::new(5).zip(CusIter::new(6)).collect();
        println!("cus_iter2: {:?}", cus_iter2);

        // 1 4 9 16 25 --> 9
        let result: u32 = CusIter::new(5)
            .zip(CusIter::new(5))
            .map(|(k, v)| k * v)
            .filter(|a| a % 3 == 0)
            .sum();
        println!("result = {}", result);

        // (1 2 3 4 5) x (2 3 4 5) = 2 6 12 20 -->filter--> 6 12 -->sum--> 18
        let result: u32 = CusIter::new(5)
            .zip(CusIter::new(5).skip(1))
            .map(|(k, v)| k * v)
            .filter(|a| a % 3 == 0)
            .sum();
        println!("result = {}", result);
    }
}

#[derive(Debug)]
pub struct Shoe {
    size: u32,
    model: String,
}

pub fn filter_by_size(vec: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    vec.into_iter().filter(|e| size == e.size).collect()
}

#[derive(Debug)]
pub struct CusIter {
    value: u32,
    curr: u32,
}

impl CusIter {
    pub fn new(value: u32) -> CusIter {
        CusIter {
            value: value,
            curr: 0,
        }
    }
}

impl Iterator for CusIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.value {
            self.curr += 1;
            Some(self.curr)
        } else {
            None
        }
    }
}
