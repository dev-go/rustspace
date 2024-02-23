fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn sum(a: i32, b: i32) -> i32 {
    println!("[sum] a = {}, b = {}, result = {}", a, b, a + b);
    a + b
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 {
            panic!("value should be greater than or equal 0: value = {}", value);
        } else if value >= 101 {
            panic!("value should be less than or equal 100: value = {}", value);
        }
        Guess { value }
    }
}

pub fn exec_long_time() -> i32 {
    println!("EXEC_LONG_TIME: >>>");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("EXEC_LONG_TIME: <<<");
    66
}

#[cfg(test)]
pub mod tests {
    // #[test]
    // pub fn test_panic() {
    //     panic!("*** PANIC in test fn ***");
    // }

    use crate::{exec_long_time, Guess};

    #[test]
    pub fn test_success() {
        let a = 100;
        let b = 66;
        let c = a + b;
        println!("a = {a}, b = {b}, c = {c}");
        assert!(c == 166, "should be success");
        assert_eq!(c, 166, "should be success");
        assert_ne!(c, 106, "should be success");
    }

    // #[test]
    // pub fn test_failed() {
    //     let a = 100;
    //     let b = 66;
    //     let c = a + b;
    //     println!("a = {a}, b = {b}, c = {c}");
    //     assert!(c == 188, "*** should be failed ***");
    //     assert_eq!(c, 188, "*** should be failed ***");
    //     assert_ne!(c, 166, "*** should be failed ***");
    // }

    // #[test]
    // #[should_panic(expected = "value should be greater than -1")]
    // pub fn test_not_panic() {
    //     let g = crate::Guess::new(166);
    //     println!("g = {:?}", g);
    // }

    #[test]
    #[should_panic(expected = "value should be less than or equal 100")]
    pub fn test_should_panic() {
        let g = crate::Guess::new(166);
        println!("g = {:?}", g);
    }

    #[test]
    pub fn test_result_ok() -> Result<(), String> {
        if 2 + 2 == 4 {
            Result::Ok(())
        } else {
            Result::Err(String::from("2 + 2 == 4 : error"))
        }
    }

    // #[test]
    // pub fn test_result_err() -> Result<(), String> {
    //     if 2 + 2 == 5 {
    //         Result::Ok(())
    //     } else {
    //         Result::Err(String::from("2 + 2 == 5: error"))
    //     }
    // }

    // #[test]
    // #[should_panic] //  functions using `#[should_panic]` must return `()`
    // pub fn test_result_err_with_should_panic() -> Result<(), String> {
    //     if 2 + 2 == 5 {
    //         Result::Ok(())
    //     } else {
    //         Result::Err(String::from("2 + 2 == 5 : error"))
    //     }
    // }

    #[test]
    #[ignore]
    pub fn test_exec_long_time() {
        let r = exec_long_time();
        assert_eq!(r, 66);
    }

    #[test]
    fn test_add() {
        let a = 1.1;
        let b = 2.1;
        let c = super::add(a, b);
        assert!(c == 3.2);
    }
}
