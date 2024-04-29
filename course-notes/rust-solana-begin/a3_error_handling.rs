/**
特点2【错误处理】

相比于其他语言常用的“抛出和捕获异常”，Rust的错误通过值返回
（从函数式编程语言中汲取的灵感）
*/


/**
---------------------------  示例：阶乘实现  -----------------------------
Java：
static int factorial(int n) {
            if (n < 0) throw new IllegalArgumentException();
            if (n == 0)
                return 1;
            else
                return n * factorial(n - 1);
        }
*/
// rust实现①：利用标记联合，它们是枚举类型，其中变体可以保存自己的数据
#![allow(unused)]
fn main() {
    enum FactorialResult {
        Valid(i32),
        InvalidArgument(i32),
    }
    fn factorial(n: i32) -> FactorialResult {
        // Assumes n is non-negative.
        fn f(n: i32) -> i32 {
            if n == 0 { 1 } else { n * f(n - 1) }
        }
        if n >= 0 {
            FactorialResult::Valid(f(n))
        } else {
            FactorialResult::InvalidArgument(n)
        }
    }
}
// rust实现②：Rust有一个通用的 Result 类型  // Ok，Err
fn factorial(n: i32) -> Result<i32, i32> {
    fn f(n: i32) -> i32 {
        if n == 0 { 1 } else { n * f(n - 1) }
    }
    if n >= 0 {
        Ok(f(n))
    } else {
        Err(n)
    }
}


