// ---------------- if ---------------
// 条件可以不需要()——parenthesis，主体一定需要{}——curly braces
fn test_if() {

    let value = 10;
    if value == 0 {
        println!("Zero!");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }

}

/**
三种循环
*/
// ---------------- loop 无限循环 -----------------
fn test_loop() {
    loop {
        println!("I can't stop!");
    }

    // 可以用 break 停止
    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!{"{i}..."};
        i -= 1;
    }
    println!("launch！");
}

// ------------------ while -------------------
fn test_while() {
    let mut i = 10;
    while i != 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");
}

// -------------------- for -------------------
// 范围表达式: a..=b
// .rev(): calls a function on the range that produces a reversed iterator
//         在生成反向迭代器的范围上调用函数。这意味着您从头到尾访问了所有项目
fn test_for() {
    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");

}


