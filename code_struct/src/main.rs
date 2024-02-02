fn main() {
    println!("Hello, World!");
    let name = String::from("liu");

    hi(&name);
    crate::hi(&name);
    // code_struct::hi(&name); // cannot find function `hi` in crate `code_struct`

    code_struct::greet(&name);
    // code_struct::crate::greet(&name); // failed to resolve: `crate` in paths can only be used in start position
    // crate::code_struct::hi(&name); // failed to resolve: could not find `code_struct` in the crate root

    // code_struct::running::run(); // module `running` is private

    code_struct::oneday::sleep();
    code_struct::oneday::eat();
    code_struct::oneday::play();
    // code_struct::oneday::play_football(); // function `play_football` is private

    let u1 = code_struct::usersys::User::new_user(String::from("liu"), String::from("12345678"));
    u1.show();

    println!("u1.name = {:?}", u1.name);
    println!("u1.name = {:?}", u1.name);
    // println!("u1.id = {:?}", u1.id); // field `id` of struct `code_struct::usersys::User` is private

    use code_struct::usersys::Role;
    let r1 = Role::Admin;
    let r2 = Role::Vip;
    println!("r1 = {:?}, r2 = {:?}", r1, r2);
}

fn hi(name: &String) -> () {
    println!("Hi, {name}!");
}
