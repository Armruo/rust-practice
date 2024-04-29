/**
【闭包】
Closures are very similar to functions, except they have the ability to "capture their environment".
闭包与函数的不同：闭包可以捕获在闭包体外面声明的变量
*/
#![allow(unused)]
fn test_closure() {

    let c = |x| {
        x * 2
    };
    println!("{}", c(6));
    // 如果闭包体是单个表达式可以省略花括号
    let c = |x| x * 2;

    // 上面的例子可以用函数fn来代替，因为它不使用它的环境
    fn c(x: i32) -> i32 {
        x * 2
    }

    // something more interesting: 每次调用c，n都会增加
    let mut n = 0;
    let mut c = |x| {
        n += 1;
        x + n - 1
    };
    println!("{}", c(2)); // 2
    println!("{}", c(2)); // 3
    println!("{}", c(2)); // 4

    // 闭包的常用场景：when iterating over collections of values
    let a = [1, 2, 3];
    let n: i32 = a.iter().map(|x| x*2).sum();
    println!("Sum of {:?} after doubling: {}", a, n);

}

fn testing_note(){
    /**
    The impl Bounds syntax in the return type means that this function should return a type that implements Bounds.
    In this case the returned type has to implement Fn(&str) which means some type that can be called as a function with a single parameter of type &str and returns nothing.
    */
    fn prefix_print(prefix: String) -> impl Fn(&str) {
        move |suffix| println!("{prefix} {suffix}")
    }

    fn main() {
        let pp = prefix_print("Hello,".to_string());

        pp("World!");
    }

    // 会打印:Hello, World
}