mod mutable;
mod scope;
mod declare;
mod freeze;


fn variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名前加上下划线以消除警告
}

#[cfg(test)]
mod variable_bindings_ground {
    use super::*;

    #[test]
    fn test_variable_bindings() {
        variable_bindings();
    }

    #[test]
    fn test_mutable() {
        mutable::mutable();
    }

    #[test]
    fn test_scope() {
        scope::scope();
    }

    #[test]
    fn test_declear() {
        declare::declear();
    }

    #[test]
    fn test_freeze() {
        freeze::freeze();
    }
    


}