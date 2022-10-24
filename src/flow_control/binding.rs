// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    50
}

fn some_number() -> Option<u32> {
    Some(50)
}

pub fn binding() {
    println!("Tell me type of person you are");

    match age() {
        // 0..12 代表 0 到 12 的值
        0 => println!("I'm not born yet I guess"),
        // 用 `n @` 来绑定值到名字 `n`。
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 不满足上面的条件，返回 `n`
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // 用 `Some(n)` 来解构 `Option`。
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        // 用 `_` 来忽略其他值。
        _ => (),
    }
}