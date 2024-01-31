// const NUM_MAX = 100; // error: missing type for `const` item
// const NUM_MIN = 0; // error: missing type for `const` item

const NUM_MAX: i32 = 100;
const NUM_MIN: i32 = 0;

fn main() {
    println!("NUM_MAX = {NUM_MAX}");
    println!("NUM_MIN = {NUM_MIN}");

    {
        println!("NUM_MAX = {NUM_MAX}");
        println!("NUM_MIN = {NUM_MIN}");

        const NUM_MAX: i32 = 200;
        const NUM_MIN: i32 = -1;

        println!("NUM_MAX = {NUM_MAX}");
        println!("NUM_MIN = {NUM_MIN}");
    }

    println!("NUM_MAX = {NUM_MAX}");
    println!("NUM_MIN = {NUM_MIN}");

    let x = 111;
    println!("x = {x}");
    // x += 100; // error[E0384]: cannot assign twice to immutable variable `x`
    println!("x = {x}");

    let mut y = 222;
    println!("y = {y}");
    y += 100;
    println!("y = {y}");

    let mut z = 333;
    println!("z = {z}");
    z += 100;
    println!("z = {z}");

    {
        println!("===>>> x = {x}, y = {y}, z = {z}");
        let a = &x;
        println!("a = {a}");
        println!("<<< x = {x}, y = {y}, z = {z}");
        println!("a = {a}");
    }

    // {
    //     println!("===>>> x = {x}, y = {y}, z = {z}");
    //     let b = &mut x; // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
    //     println!("b = {b}");
    //     println!("<<< x = {x}, y = {y}, z = {z}");
    //     println!("b = {b}");
    // }

    {
        println!("===>>> x = {x}, y = {y}, z = {z}");
        let c = &y;
        println!("c = {c}");
        // *c += 100; // error[E0594]: cannot assign to `*c`, which is behind a `&` reference
        println!("<<< x = {x}, y = {y}, z = {z}");
        println!("c = {c}");
    }

    {
        println!("===>>> x = {x}, y = {y}, z = {z}");
        let d = &mut y;
        println!("d = {d}");
        // println!("<<< x = {x}, y = {y}, z = {z}"); // error[E0502]: cannot borrow `y` as immutable because it is also borrowed as mutable
        *d += 100;
        println!("d = {d}");
        println!("<<< x = {x}, y = {y}, z = {z}");
    }

    {
        println!("===>>> x = {x}, y = {y}, z = {z}");
        let mut e = &mut y;
        println!("e = {e}");
        // println!("<<< x = {x}, y = {y}, z = {z}"); // eerror[E0502]: cannot borrow `y` as immutable because it is also borrowed as mutable
        *e += 100;
        println!("e = {e}");

        // e = &x; // error[E0308]: mismatched types    note: expected mutable reference `&mut {integer}`, found reference `&{integer}`

        // e = &mut x; // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable

        // e = &z; // error[E0308]: mismatched types    note: expected mutable reference `&mut {integer}`, found reference `&{integer}`

        println!("<<< x = {x}, y = {y}, z = {z}");

        e = &mut z;
        println!("e = {e}");
        *e += 100;
        println!("e = {e}");

        println!("<<< x = {x}, y = {y}, z = {z}");
    }
}
