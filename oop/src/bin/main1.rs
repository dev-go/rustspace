fn main() {
    println!("*** *** OOP *** ***");

    {
        println!("*** *** *** ***");
        let mut avg_list = crate::AvgList::new();
        println!("avg_list = {:?}", avg_list);
        avg_list.push(99);
        println!("avg_list = {:?}", avg_list);
        avg_list.push(98);
        println!("avg_list = {:?}", avg_list);
        avg_list.push(97);
        println!("avg_list = {:?}", avg_list);

        let e = avg_list.pop();
        println!("e = {:?}", e);
        println!("avg_list = {:?}", avg_list);

        let e = avg_list.pop();
        println!("e = {:?}", e);
        println!("avg_list = {:?}", avg_list);

        let e = avg_list.pop();
        println!("e = {:?}", e);
        println!("avg_list = {:?}", avg_list);

        let e = avg_list.pop();
        println!("e = {:?}", e);
        println!("avg_list = {:?}", avg_list);
    }
}

#[derive(Debug)]
pub struct AvgList {
    list: Vec<i32>,
    avg: f64,
}

impl AvgList {
    pub fn new() -> AvgList {
        AvgList {
            list: vec![],
            avg: 0.0,
        }
    }

    pub fn push(&mut self, elem: i32) {
        self.list.push(elem);
        self.update_avg();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let elem = self.list.pop();
        self.update_avg();
        elem
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        if self.list.len() == 0 {
            self.avg = 0.0;
        } else {
            let sum: i32 = self.list.iter().sum();
            let avg = sum as f64 / self.list.len() as f64;
            self.avg = avg;
        }
    }
}
