fn main() {
    // 所有权

    // s不可用
    let mut s = 1; //s可用
    s += 1; // 操作
    println!("{}", s);

    // 使用String的from 来定义字符串使其能被修改 存在堆上
    // 而字面量的字符串则因为长度和内容固定会存在栈中
    let mut s = String::from("你好");
    s.push_str(",世界！");
    println!("{}", s);

    // 数据交互方式
    // move 移动
    // 简单数据类型 压入stack中
    let z = 5;
    let s = z;
    println!("{}", s);
    // 简单数据类型 复制操作
    println!("{}", z);

    // 复杂数据类型
    let z1 = String::from("5");
    let s = z1;
    println!("{}", s);
    // 指针存在于栈中 而数据存储在堆上
    // 这里不能调用的原因是因为 z1 的指针已经从z1 移动到了 s 
    // 而被移动的指针则会直接remove(移除/废弃) 从而z1的指针无法使用
    // 如果想要使用z1则只有使用: let s = z1.clone()  让s在栈和堆上克隆一份z1的数据 
    // println!("{}", z1); // error orrow of moved value: `z1`

} //s 离开scope 不可用
