/**
【引用】允许你使用值，但是不获取所有权
①符号：
  引用：&T 或 &mut T
  解引用：*
②可变引用与不可变引用 不能同时存在
*/


/**
------------------------------ 不可变引用 ----------------------------
①对 Type 的不可变引用是 &Type
②对一个值可以有多个不可变引用
*/
struct Config {
    port: u16,
}
fn immutable_refs() {
    let config: Config = Config {
        port: 8080
    };
    // ①
    let config_refs: &Config = &config;
    println!("Using port {}.", config_refs.port);

    // ②
    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be the same as {r2}");
}

/**
--------------------------------------- 可变引用 --------------------------
①对 Type 的可变引用是 &mut T
②对一个值同时只能有一个可变引用
*/
fn mutable_refs() {
    let mut config: Config = Config{
        port: 8080
    };
    // ①
    let config_refs: &mut Config = &mut config;
    config_refs.port = 4000;
    println!("Using port {}.", config.port);

    // ②
    let mut val = 10;
    let r1 = &mut val;
    let r2 = &mut val;
    *r1 = 5;
    *r2 = 6;
}

/**
------------------ 解除引用 -----------------
①用 * 获取“the value behind a reference”
*/
fn de_ref_operator() {
    let val: i32 = 10;
    let r1: &i32 = &val;
    // This creates a copy of the value 10.
    // it works because i32 is copyable
    let val2: i32 = *r1;
}

/**
------------------- copy ------------------
① 比如 String 是不可复制的，因为它包含了指向堆中某些内存的指针
*/

/**
---------------- 生命周期 Lifetimes ---------------
①引用有个所谓的生命周期
②描Lifetimes述了在代码的哪一部分可以安全地使用引用。
  虽然大多数时候，编译器可以自动推断所有引用的生命周期，但明确描述意图有时会很有用。
③ 表示：'lifetime ；一般可省略，编译器知道。
*/



