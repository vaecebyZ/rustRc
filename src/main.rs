fn main() {
    // 不持有所有权的类型 切片
    let mut str = String::from("hello world");
    let res = first_word(&str);
    str.clear();
    println!("{}", res)
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
