/**
特点3【零成本抽象】
abstractions：reducing the complexity of a problem
“cost”：the run-time penalty

Abstraction例子之一：迭代器 iterator
使用迭代器和迭代器适配器（iterator adapter），而不是手动跟踪索引并在过滤时有条件地执行循环体来编写循环
*/

/**
总结：
Rust 提供了实现抽象所需的工具，而运行时开销很少或没有
Rust provides the tools necessary to implement abstractions that have little or no run-time overhead.

为了充分利用这一点，您可能需要了解：
泛型、零大小类型、编译器优化、不安全和其他主题
generics, zero sized types, compiler optimizations, unsafe and other topics.
*/
