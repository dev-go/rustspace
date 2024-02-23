pub mod common;

#[test]
pub fn test_add() {
    // crate::common::setup();
    common::setup();

    // let a = testdemo::add(1.1 , 2.2); // function `add` is private
    let b = testdemo::sum(100, 66);
    println!("b = {b}");
    assert_eq!(b, 166);
}
