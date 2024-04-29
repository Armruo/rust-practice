/**
【泛型】
*/

impl Sequence3_I32 {
    pub fn new(first: i32, second: i32, third: i32) -> Self {
        Self {first, second, third}
    }
}
impl Sequence3_String {
    pub fn new(first: String, second: String, third: String) -> Self {
        Self { first, second, third }
    }
}

//-----------------------

struct Sequence3<T> {
    first: T,
    second: T,
    third: T,
}

impl<T> Sequence3<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self {first, second, third}
    }
}

// ------------------ Traits ----------------

use std::cmp::PartialEq;
// For all types T implementing PartialEq, implement for Sequence3<T> ...
impl <T: PartialEq> Sequence3<T> {
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}
//
impl<T> Sequence3<T> where T: PartialEq {
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}

// ------------------- Associated Types in Trait Bounds ---------------
// 实现一个sum操作，把Sequence3中所有的值求和
use std::ops::Add;
impl<T> Sequence3<T> where T: Copy + Add<Output = T> {
    pub fn sum(&self) -> T {
        self.first + self.second + self.third
    }
}

// --------------- Using Multiple Generic Type Parameters --------------
// todo 类型也可以take多种泛型
struct MyStruct<A, B> { a: A, b: B, }
enum MyEnum<A, B> { A(A), B(B), }
fn main() {
    let s = MyStruct {
        a: 10,
        b: "str"
    };
    // We have to specify the type of the `MyEnum::A` variant here because Rust does not have
    // information to infer it
    let e = MyEnum::<i32, _>::B("Hello");
}

// -------------- Generic Functions ---------------
// 也可以编写接受泛型类型的自由函数：


