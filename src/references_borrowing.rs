fn main() {
    let strs = String::from("你好,世界");

    let new = new_function(strs);

    println!("{}", new);

    // println!("{}",strs); // err: strs was moved

    let strs = String::from("你好,世界");

    let new = new_next_function(&strs);

    println!("{},{}", strs, new); //its work
    
    //& 标识传入引用所有权不发生移动只是复制了一个指向本来数据的指针的指针
    // 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。 如果需要就需要在所有关联代码中加mut关键字

    // 可变引用只能在同一位置发生修改不能二次引用
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; error
    println!("{}", r1);

    // 也不可以将一个引用二次引用同时存在可变和不可变

    let s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    println!("{},{}", r1, r2);


}

fn new_function(x: String) -> String {
    println!("{}", x);
    String::from("世界")
}

// &str标识
fn new_next_function(x: &str) -> &str {
    x
}
