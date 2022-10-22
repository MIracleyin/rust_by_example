
#![allow(unused)]

use std::fmt;
fn main() {
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
}

// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    // 这个 trait 要求实现一个 `fmt` 函数，它接受一个指向 `self` 的引用，同时接受一个
    // `&mut fmt::Formatter` 实例作为参数。这个函数将格式化后的字符串写入第二个参数中，
    // 并返回一个 `fmt::Result` 来表示是否成功。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.0` 来访问第一个字段。
        write!(f, "{}", self.0)
    }
    
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


pub fn debug() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    // println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
