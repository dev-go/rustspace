/*
1. 闭包:
    (1) 不捕获任何环境中变量的闭包可以转换为函数指针
    (2) 闭包的方法通过不可变引用访问其捕获的变量的闭包实现了Fn
    (3) 闭包的方法通过可变引用访问其捕获变量的闭包实现了FnMut
    (4) 闭包的方法若获取了只能被调用一次，即实现了FnOnce

2. 闭包在Rust中的实现可以近似地理解为一个实现了FnOnce、FnMut和Fn其中一个trait的匿名结构体，
这个匿名结构体保存捕获的环境中的变量。通过调用trait的方法来执行闭包体中的代码。
    // FnOnce
    #[lang = "fn_once"]
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnOnce<Args> {
        type Output;
        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    }

    // FnMut
    #[lang = "fn_mut"]
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnMut<Args>: FnOnce<Args> {
        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    }

    // Fn
    #[lang = "fn"]
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait Fn<Args>: FnMut<Args> {
        extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    }

Fn是FnMut的子trait，FnMut是FnOnce的子trait。也就是说实现了Fn的闭包一定实现了FnMut，同样，实现了FnMut的闭包一定实现了FnOnce。

3. 闭包实现这三个trait的规则如下：
    (1) 所有的闭包都实现了FnOnce
    (2) 如果闭包的方法移出了所捕获的变量的所有权，则只会实现FnOnce
    (3) 如果闭包的方法没有移出所捕获的变量的所有权，并且对变量进行了修改，即通过可变借用使用所捕获的变量，则会实现FnMut
    (4) 如果闭包的方法没有移出所捕获的变量的所有权，并且没有对变量进行修改，即通过不可变借用使用所捕获的变量，则会实现Fn。

4. 关键字 move 的作用是将所引用的变量的所有权转移至闭包内，通常用于使闭包的生命周期大于所捕获的变量的原生命周期（例如将闭包返回或移至其他线程）。

5. 闭包捕获环境中变量的模式为优先不可变借用，而后依次为唯一不可变借用（例如&&mut T)，可变借用，移动。

6. 当闭包借用环境中的变量时，引用变量 &T（或 &mut T）将保存在闭包匿名结构体中。
此时若想获取所引用的变量的所有权，就要使用move关键字将其所有权转移至闭包中，闭包会捕获所引用的变量本身 T（或 mut T）。
当闭包移动环境中的变量时，闭包会根据其语义进行 Move 或 Copy。捕获的变量T（或 mut T）将保存在闭包匿名结构体中。

7. 捕获模式
    (1) 闭包捕获环境中变量的模式为优先不可变借用，而后依次为唯一不可变借用（例如&&mut T)，可变借用，移动。
    (2) 当闭包借用环境中的变量时，引用变量&T（或&mut T）将保存在闭包匿名结构体中。
此时若想获取所引用的变量的所有权，就要使用move关键字将其所有权转移至闭包中，闭包会捕获所引用的变量本身T（或mut T），也就是下面所要说的情况。
    (3) 当闭包移动环境中的变量时，闭包会根据其语义进行Move或Copy。捕获的变量T（或mut T）将保存在闭包匿名结构体中。
    (4) 简单地说，如果闭包捕获的变量为引用&T(或&mut T），使用关键字move后，闭包会根据所引用的对象的语义（Copy或Move）捕获T(或mut T)。

8. 闭包实现FnOnce、FnMut和Fn中的哪个trait只与闭包如何使用所捕获的变量有关，与如何捕获变量无关。
因此，关键字move不影响闭包实现FnOnce、FnMut和Fn。关键字move主要用于使闭包摆脱所捕获的变量的生命周期限制，例如将闭包返回或移至其他线程时，必须使用move。
*/

fn main() {
    println!("*** *** Concurrent *** ***");

    {
        println!("===============");
        let num = 10;
        println!("MAIN: num = {:?}", num);

        let f1 = || {
            println!("===>>> f1: num = {:?}", num);
        };
        // let join1 = std::thread::spawn(f1); // closure may outlive the current function, but it borrows `num`, which is owned by the current function

        let f2 = || {
            println!("===>>> f2: num = {:?}", num);
            num
        };
        // let join2 = std::thread::spawn(f2); // closure may outlive the current function, but it borrows `num`, which is owned by the current function

        let f3 = move || {
            println!("===>>> f3: num = {:?}", num);
        };
        println!(
            "===>>f3 result: {:?}",
            std::thread::spawn(f3).join().unwrap()
        );

        let f4 = move || {
            println!("===>>> f4: num = {:?}", num);
            return num;
        };
        println!(
            "===>>f4 result: {:?}",
            std::thread::spawn(f4).join().unwrap()
        );
    }

    {
        println!("===============");
        let mut num = 10;
        println!("MAIN: num = {:?}", num);

        let f1 = move || {
            println!("===>>> f1: num = {:?}", num);
            num += 100;
            println!("===>>> f1: num = {:?}", num);
        };

        let f2 = move || {
            println!("===>>> f1: num = {:?}", num);
            num += 1000;
            println!("===>>> f1: num = {:?}", num);
        };

        let f1_result = std::thread::spawn(f1);
        let f2_result = std::thread::spawn(f2);

        println!("MAIN: num = {:?}", num);
        f1_result.join();
        f2_result.join();
        println!("MAIN: num = {:?}", num);
    }

    {
        println!("===============");
        let mut s = String::from("abc");
        println!("MAIN: s = {:?}", s);
        let f1 = || {
            println!("===>>> s = {:?}", s);
        };
        let f2 = move || {
            println!("===>>> s = {:?}", s);
        };
        // let f3 = move || { // use of moved value: `s`
        //     println!("===>>> s = {:?}", s);
        // };
        // println!("MAIN: s = {:?}", s); // borrow of moved value: `s`
        let f2_result = std::thread::spawn(f2);
        // let f3_result = std::thread::spawn(f3);
        f2_result.join();
        // println!("MAIN: s = {:?}", s); // borrow of moved value: `s`
    }

    println!("=====================================");

    let num = 10;
    println!("MAIN: num = {:?}", num);

    let thread_func = move || {
        println!("--->>> CLOSURE start >>>");
        let mut sum = 0;
        for i in 1..=num {
            sum += i;
            println!("--->>> CLOSURE: i = {}, sum = {}", i, sum);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("--->>> CLOSURE finished <<<");
        return sum;
    };
    let join_handle = std::thread::spawn(thread_func);

    println!("MAIN: num = {:?}", num);

    for i in 0..5 {
        println!("MAIN: i = {i}");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let thread_result = join_handle.join();
    println!("MAIN: thread_result = {:?}", thread_result);

    for i in 0..5 {
        println!("MAIN: i = {i}");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("MAIN: over!");
}
