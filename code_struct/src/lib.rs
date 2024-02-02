pub fn greet(name: &String) {
    println!("greet");
    // code_struct::priv_hello(name); // failed to resolve: use of undeclared crate or module `code_struct`
    crate::priv_hello(name);
}

fn priv_hello(name: &String) {
    println!("Hello, {name}!");
}

mod running {
    pub fn run() {
        println!("running::run");
    }
}

pub mod oneday {
    pub fn sleep() {
        println!("oneday::sleep");
    }

    pub fn eat() {
        println!("oneday::eat");
    }

    pub fn play() {
        println!("oneday::play");
        play_football();
        crate::oneday::play_watch_tv();
    }

    fn play_watch_tv() {
        println!("===>>> play_watch_tv");
    }

    fn play_football() {
        println!("===>>> play_football");
        crate::running::run();
    }
}

pub mod front_of_house;


pub mod usersys;