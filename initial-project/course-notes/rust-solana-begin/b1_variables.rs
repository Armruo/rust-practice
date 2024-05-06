/**
【变量】
①声明：不可变变量 let；可变变量 let mut
②希望不要警告未使用的变量：在变量名前加一个下划线_
*/
fn variables() {
    // 不可变
    let x = 2;
    // 可变
    let mut y = -3;
}

// ----------- 作用域（scope） {} ------------------------------------------------------------
/**
--------------- 【变量遮蔽（shadowing）】: allowed to redefine a variable with the same name -----
变量遮蔽的用处在于：
如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），
就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
*/
fn scope_shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 6

    /** 注
    🔺这和 mut 变量的使用是不同的，
    第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
    而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    */
}

/**
------------------- 模式Patterns ------------------
destructuring a tuple 结构元组：
let (x, y) = (2, 3);
*/
// destructure structs：
fn pattern() {
    // 自定义一个Person类型
    struct Person {
        name: &'static str,
        age: u32,
        likes: bool,
    }
    // 创建
    let p = Person {
        name: "Mick",
        age: 30,
        likes: true,
    };
    // 分解 deconstruct
    let Person {
        name, age, ..  // 🔺".."用于表示“任何剩余字段”，允许您仅关心您指定的字段
    } = p;
}
