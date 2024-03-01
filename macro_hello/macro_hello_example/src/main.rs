fn main() {
    println!("*** *** Macro *** ***");

    use macro_hello::MacroHello;
    use macro_hello_impl::MacroHello;

    #[derive(MacroHello)]
    struct Pancakes;

    Pancakes::macro_hello();
}
