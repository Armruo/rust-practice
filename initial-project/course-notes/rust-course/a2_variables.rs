/**
【变量解构】
let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：
从一个相对复杂的变量中，匹配出该变量的一部分内容
*/
fn main() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

/**
【解构式赋值】
ps: 使用 += 的赋值语句还不支持解构式赋值
*/
struct StructA{
    e: i32
}
fn main_2() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1,2,3,4,5];
    StructA {e, ..} = StructA {e:5};

    assert_eq!([1, 2, 1, 4, 6], [a, b, c, d, e]);
}