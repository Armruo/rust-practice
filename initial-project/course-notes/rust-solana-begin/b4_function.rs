/**
【函数】
函数声明：
fn fn_name(input1: InputType1, input2, InputType2) -> OutputType {
    // body
}
*/
fn u32_add(a: u32, b: u32) -> u32 {
    return a + b; // return和分号可以同时省略
}

/**
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