pub fn whiles() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut i = 1;
    let mut vec = Vec::new();
    while i < n {
        if i % 15 == 0 {
            vec.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            vec.push("Fizz".to_string());
        } else if i % 5 == 0 {
            vec.push("Buzz".to_string());
        } else {
            vec.push(i.to_string());
        }

    }
    vec
}