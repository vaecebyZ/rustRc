fn main() {
    // 不持有所有权的类型 切片
    let mut str = String::from("hello world");
    let res = first_word(&str);
    str.clear(); //res 和 我们的 字符串 str并不相关联 所以即使 str被清空了 res仍然能正常访问；
    println!("{}", res);

    // ------------切片方式--------------
    // 定义切片
    let str = String::from("hello world");
    println!("{}", str);
    // 如果从第一个开始则可以省略..开头的数字 结尾同理
    let sstr = &str[..2]; //切片指向字符串一部分
    println!("{}", sstr);

    // 重写first_word
    let res = get_word(&str);
    // res.clear(); //error
    println!("{}", res);
}

fn first_word(s: &String) -> usize {
    let str_byte = s.as_bytes();

    // iter 返回一个可以被枚举的数据集合 enumerate 返回经过包装的元组的
    for (i, &item) in str_byte.iter().enumerate() {
        // 模式匹配

        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_word(moji: &str) -> &str {
    let str_byte = moji.as_bytes();

    // iter 返回一个可以被枚举的数据集合 enumerate 返回经过包装的元组的
    for (i, &item) in str_byte.iter().enumerate() {
        // 模式匹配
        if item == b' ' {
            return &moji[..i];
        }
    }

    &moji[..]
}
