/**
【闭包】
Closures are very similar to functions, except they have the ability to "capture their environment".
*/

fn test_closure() {
    let c = |x| {
        x * 2
    };
    println!("{}", c(6))
}