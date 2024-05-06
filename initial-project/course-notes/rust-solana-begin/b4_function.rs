/**
【函数】
①函数声明：
fn fn_name(input1: InputType1, input2, InputType2) -> OutputType {
    // body

②返回值不包含分号
③永不返回的发散函数(diverge function)：用 ! 作函数返回类型，这种语法往往用做会导致程序崩溃的函数
}
*/
fn u32_add(a: u32, b: u32) -> u32 {
    return a + b; // return和分号可以同时省略
}

/**
----------------- method ------------------
语法：T::fn_name(args...)
*/
fn test_method() {
    struct X(&'static str);

    impl X {
        // An implementation block for the type `X`.
        fn associated_fn() -> &'static str {
            "same"
        }
        // A method.
        fn method(self: &Self) -> &'static str {  // 可以简写为method(&self)
            self.0 // Self 或 &Self 或 &mut Self
        }
    }

    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`");
    println!("{}", instance.method());
}