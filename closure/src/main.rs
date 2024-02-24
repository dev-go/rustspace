fn main() {
    println!("*** Closure ***");

    {
        println!("*** *** *** ***");
        workout(10, 6);
    }

    {
        println!("*** *** *** ***");
        workout_closure(10, 6);
    }

    {
        println!("*** *** *** ***");
        workout_closure_opt(10, 6);
    }

    {
        println!("*** *** *** ***");
        let base = vec![String::from("a"), String::from("b"), String::from("c")];
        let func = |v: Vec<String>| {
            let mut m = std::collections::HashSet::new();
            for e in &base {
                m.insert(e);
            }
            for e in &v {
                if !m.contains(e) {
                    return false;
                }
            }
            return true;
        };
        println!("base = {:?}", base);
        let v1 = vec![String::from("a"), String::from("b")];
        let v2 = vec![String::from("a"), String::from("b"), String::from("d")];
        let r1 = func(v1);
        let r2 = func(v2);
        // let r3 = func(v2); // use of moved value: `v2`
        println!("r1 = {r1}, r2 = {r2}");
        println!("base = {:?}", base);
    }

    {
        println!("*** *** *** ***");
        let base = vec![String::from("a"), String::from("b"), String::from("c")];
        let func = move |v: Vec<String>| {
            let mut m = std::collections::HashSet::new();
            for e in &base {
                m.insert(e);
            }
            for e in &v {
                if !m.contains(e) {
                    return false;
                }
            }
            return true;
        };
        // println!("base = {:?}", base); // borrow of moved value: `base`
        let v1 = vec![String::from("a"), String::from("b")];
        let v2 = vec![String::from("a"), String::from("b"), String::from("d")];
        let r1 = func(v1);
        let r2 = func(v2);
        println!("r1 = {r1}, r2 = {r2}");
    }

    {
        println!("*** *** *** ***");
        let u = User {
            id: 1001,
            name: String::from("user_1001"),
        };
        let f = || {
            println!("f : u = {:?}", u);
            let u2 = { ..u };
            println!("f : u2 = {:?}", u2);
        };
        func_once(f);
        // func_once(f); // use of moved value: `f`
    }

    {
        println!("*** *** *** ***");
        let mut u = User {
            id: 1001,
            name: String::from("user_1001"),
        };
        let f = || {
            println!("f : u = {:?}", u);
            u.name = String::from("liu");
            println!("f : u = {:?}", u);
        };
        func_mut(f);
        // func_mut(f); // use of moved value: `f`
    }
}

#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
}

impl Drop for User {
    fn drop(&mut self) {
        println!("USER drop: id = {}, name = {:?}", self.id, self.name);
    }
}

pub fn func_once<F>(func: F)
where
    F: FnOnce() -> (),
{
    println!("func_once: --->>>");
    func();
    // func(); // use of moved value: `func`
    println!("func_once: ---<<<");
}

pub fn func_mut<F>(mut func: F)
where
    F: FnMut() -> (),
{
    println!("func_mut: --->>>");
    func();
    func();
    println!("func_mut: ---<<<");
}

// pub fn func_mut2<F>(func: F) // cannot borrow `func` as mutable, as it is not declared as mutable
// where
//     F: FnMut() -> (),
// {
//     println!("func_mut2: --->>>");
//     func();
//     func();
//     println!("func_mut2: ---<<<");
// }

#[derive(Debug)]
pub struct Workout<T: Fn(u32) -> u32> {
    func: T,
    cache: std::collections::HashMap<u32, u32>,
}

impl<T: Fn(u32) -> u32> Workout<T> {
    pub fn new(func: T) -> Workout<T> {
        Workout {
            func: func,
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        let r = self.cache.get(&arg);
        match r {
            Some(v) => *v,
            None => {
                let r = (self.func)(arg);
                self.cache.insert(arg, r);
                r
            }
        }
    }
}

fn workout_closure_opt(intensity: u32, random: u32) {
    println!(
        "workout_closure_opt: intensity = {}, random = {}",
        intensity, random
    );
    // let mut workout = Workout::new(workout_expensive);
    let mut workout = Workout::new(|intensity| {
        println!("workout_closure_opt::func: >>>");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("workout_closure_opt::func: <<<");
        intensity
    });
    if intensity < 25 {
        // 1) pushups
        println!(
            "workout_closure_opt: 1) do {} pushups",
            workout.value(intensity)
        );
        // 2) situps
        println!(
            "workout_closure_opt: 2) do {} situps",
            workout.value(intensity)
        );
    } else {
        if random % 3 == 0 {
            // take a break
            println!("workout_closure_opt: take a break");
        } else {
            // run
            println!(
                "workout_closure_opt: run {} minutes",
                workout.value(intensity)
            );
        }
    }
}

fn workout_closure(intensity: u32, random: u32) {
    println!(
        "workout_closure: intensity = {}, random = {}",
        intensity, random
    );
    let f = || {
        println!("workout_closure::f: >>>");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("workout_closure::f: <<<");
        return intensity;
    };
    if intensity < 25 {
        // 1) pushups
        println!("workout_closure: 1) do {} pushups", f());
        // 2) situps
        println!("workout_closure: 2) do {} situps", f());
    } else {
        if random % 3 == 0 {
            // take a break
            println!("workout_closure: take a break");
        } else {
            // run
            println!("workout_closure: run {} minutes", f());
        }
    }
}

fn workout(intensity: u32, random: u32) {
    println!("workout: intensity = {}, random = {}", intensity, random);
    if intensity < 25 {
        // 1) pushups
        println!("workout: 1) do {} pushups", workout_expensive(intensity));
        // 2) situps
        println!("workout: 2) do {} situps", workout_expensive(intensity));
    } else {
        if random % 3 == 0 {
            // take a break
            println!("workout: take a break");
        } else {
            // run
            println!("workout: run {} minutes", workout_expensive(intensity));
        }
    }
}

fn workout_expensive(intensity: u32) -> u32 {
    println!("workout_expensive: >>>");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("workout_expensive: <<<");
    return intensity;
}
