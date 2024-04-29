/**
【模式匹配】
Pattern matching：deconstruct values of complex types into their parts

<----  可驳模式 ---->
条件匹配时，模式不必包含所有可能性
<----- 不可驳模式 ----->
其他时候，如用let声明变量时，模式必须是无可辩驳的，这意味着该模式可以匹配您要匹配的类型的任何可能值。
let的结构：let <pattern>: <type> = <expression>; <type>和<expression>可省略
*/

fn test_pattern_match() {
    // 定义
    struct Plant {
        flowering: bool,
        mass: f64
    }
    let Plant { flowering, mass};

    // 初始化
    let p = Plant {
        flowering: true,
        mass: 10.0,
    };
    let Plant {flowering, mass} = p;
}
// ---------------- enum match -----------------
fn test_enum() {
    enum Meal {
        FishAndChips {chip_proportion: f64},
        Hamburger {vegetarian: bool},
    }

    let m = Meal::Hamburger {vegetarian: true};

    if let Meal::Hamburger {..} = m {
        println!("I had a hamburger!");
    }


    if let Meal::Hamburger { vegetarian: true } = m {
        println!("I had a vegetarian hamburger!");
    }

    // 以下程序打印：Jumping
    enum Key { Up, Down, Left, Right };
    match Key::Left {
        Key::Up => println!("Jumping"),
        Key::Down => println!("Ducking!"),
        Key::Left => println!("Sliding Left!"),
        Key::Right => println!("Sliding Right!"),
    }

}

fn test_match_0() {
    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 => println!("It was a high number!"), // 🔺 match guard
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }
}

// ----------------- if let 中的模式：表达式类型中必须存在模式未涵盖的值 ------------------
fn test_match_if() {

    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!");
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }
}

// ------------ while let 循环，直到模式不匹配为止 ---------------
fn test_while_let() {
    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };

    while let Meal::FishAndChips { chip_proportion } = meal {
        println!("Having fish and chips with chip proportion {:.2}...", chip_proportion);
        if chip_proportion > 0.3 {
            // Order a meal with less chips.
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            // Too fishy, let's get a hamburger next.
            meal = Meal::Hamburger { vegetarian: true }
        }
    }
    println!("I'm so done with fish and chips!");
}

// ---------------- for 不可驳模式 ----------------
fn test_for() {
    let tuples: [(usize, &'static str); 3] = [
        (1, "red"),
        (2, "white"),
        (3, "blue"),
    ];
    for (numbering, color) in tuples {
        println!("color #{numbering} is {color}");
    };

    // 使用std::Iterator::enumerate（https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate）
    let colors = ["red", "white", "blue"];
    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("color #{numbering} is {color}")

    }

    // 以下程序打印：abb
    for n in 1..=3 {
        match n {
            0 | 1 => print!("a"),
            _ => print!("b"),
            other if other > 2 => print!("c"),
        }
    }
}