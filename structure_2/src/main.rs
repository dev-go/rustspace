#[derive(Debug)]
struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    pub fn square(length: u32) -> Rect {
        Rect {
            width: length,
            height: length,
        }
    }
}

fn main() {
    let mut r1 = Rect {
        width: 4,
        height: 3,
    };
    println!("r1 = {:?}", r1);
    let r1_area = rect_area(&r1);
    println!("r1_area = {:?}", r1_area);
    r1.height *= 2;
    println!("r1 = {:?}", r1);
    let r1_area = rect_area(&r1);
    println!("r1_area = {:?}", r1_area);

    let mut r2 = Rect {
        width: 12,
        height: 5,
    };
    println!("r2 = {:?}", r2);
    let r2_area = r2.area();
    println!("r2_area = {:?}", r2_area);
    r2.width *= 2;
    println!("r2 = {:?}", r2);
    let r2_area = r2.area();
    println!("r2_area = {:?}", r2_area);
    r2.double();
    println!("r2 = {:?}", r2);
    let ok = r2.can_hold(&r1);
    println!("ok = {ok}");

    let r3 = Rect::square(5);
    println!("r3 = {:?}", r3);
    let ok = r2.can_hold(&r3);
    println!("ok = {ok}");
}

fn rect_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
