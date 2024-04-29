/**
特点1【数据所有权】
Rust在语言中内置了数据所有权的概念，这是它独一无二的点

以下实现返回一个指向内存的指针，该指针在程序开始时加载到内存中一次，并在程序的整个持续时间内保留在那里
char* determine_thing() {
return "THING";
}

使用stdup：
char* determine_thing_2() {
return strdup("THING");
}
*/

// 等效的 Rust 实现如下所示
fn determine_thing() -> &'static str {
    "THING"
}
fn determine_thing_2() -> String {
    String::from("THING")
}

/**
上例中的两种返回类型
&'static str：表示对在程序的整个运行期间都存在的字符串的引用
String：表示一个字符串，该字符串的所有权已移出函数，并返回给调用者
*/