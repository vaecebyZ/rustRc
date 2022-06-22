fn main() {
    println!("im main.");
    let a = 1;
    anothor(a);
    println!("5---{}", a);

    // 表达式
    // 块级作用域如果最后一行代码为表达式且没有分号则这个块级作用域的值也为最后一行表达式的值.
    let x = {
        let d = 1;
        d + 1 //type is () tuple
    };
    println!("块级表达式{}", x);
    println!("函数返回值{}", has_return(3));
}

// 形参必须指明类型
fn anothor(mut a: i32) {
    a = a + 1;
    println!("10---{}", a);
    println!("im anothor function.");
}

// 使用->指定返回值类型
fn has_return(a: i32) -> i32 {
    // 最后一个表达式 (不能加分号)
    a + 1
    // 也可以
    // return a+1
}
