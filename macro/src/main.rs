fn main() {
    println!("*** *** Macro *** ***");
    {
        #[macro_export]
        macro_rules! my_vec {
            ($($x: expr), *) => {
                {
                    let mut temp = Vec::new();
                    $(
                        temp.push($x);
                    )*
                    temp
                }
            };
        }

        let v1 = my_vec!(11, 22, 33);
        println!("v1: {:?}", v1);
        let v2 = my_vec![11, 22, 33];
        println!("v2: {:?}", v2);
    }
}
