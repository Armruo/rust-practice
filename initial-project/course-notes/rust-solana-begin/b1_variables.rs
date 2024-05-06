/**
可变变量与不可变变量
声明：let；let mut
希望不要警告未使用的变量：在变量名前加一个下划线_
*/
fn variables() {
    // 不可变
    let x = 2;
    // 可变
    let mut y = -3;
}

// ----------- scope作用域 ------------
fn scope() {
    let x = 2;
    println!("{}", x);

    // 创建一个新的作用域
    {
        let y = 3;
        // 这里可以调用x
        println!("{}", x); // 2
        println!("{}", y); // 3
    }
    println!("{}", x); //2
    println!("{}", y); // won't compile because y is "not in scope"
}

// --------- Shadowing: allowed to redefine a variable with the same name -------
fn shadowing() {
    let x = 2;
    let x = 3;

    // 结合作用域
    let x = 2;
    {
        let x = 3;
    }
    println!("{}", x); // 2  🔺如果x定义是let mut x可变变量，则这里输出的是3
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
