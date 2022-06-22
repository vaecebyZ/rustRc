//data type
fn main() {
    // scalar 标量类型
    //  * 整型
    //  * 浮点型
    //  * 布尔类型
    //  * 字符类型
    //  -----------

    // 整数类型
    // --是一个没有小数部分的数字。
    // exp: i8 u8 ... i128 u128 ... isize usize
    // 默认i32
    // isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的

    let a = b'A'; // Byte 字符
    println!("Byte,{}", a);
    let a = 123_321; // Decimal 十进制
    println!("Decimal,{}", a);
    let a = 0xffff; // Hex 十六进制
    println!("Hex,{}", a);
    let a = 0o7723; // Octal 八进制
    println!("Octal,{}", a);
    let a = 0b11100_1010; // Bunary 二进制
    println!("Bunary,{}", a);

    // 浮点型
    // --是一个有小数部分的数字。
    // exp:f32 f64
    // 默认 f64

    let a = 1.2345;
    println!("f64,{}", a);
    let a: f32 = 1.2345;
    println!("f32,{}", a);

    // --数值操作
    // +-*/%

    // 注意！ 整型和浮点不可以一起参与计算
    let a = (1 as f64) + 1.23;
    println!("{}", a);

    let a = (1 as f64) - 1.23;
    println!("{}", a);

    let a = (1 as f64) * 1.23;
    println!("{}", a);

    let a = (1 as f64) / 1.23;
    println!("{}", a);

    let a = (1 as f64) % 1.23;
    println!("{}", a);

    // 布尔 占用一个字节
    let a = true;
    println!("{}", a);
    let a: bool = false;
    println!("{}", a);

    // 字符 char 类型是语言中最原生的字母类型，使用单引号。
    let a = 'z';
    println!("{}", a);
    let a = 'ℤ';
    println!("{}", a);
    let a = '😻';
    println!("{}", a);

    // --------------------------------------------------
    // compound types 复合类型
    //  * 元组 tuple
    //  * 数组 array
    // 可以将多个值放入一个类型中
    //  -----------

    // tuple
    // 长度固定声明后无法修改 多个类型的值可以放在一个tuple中
    // 使用()标识

    // 显式赋值
    let a: (i32, f64, char) = (1, 1.12, '一');

    // 类型推断
    let tup = (1, 1.12, '一');

    // 标记法
    println!("tup取值{}", tup.2);

    // 解构
    let (b, c, d) = a;
    println!("b是{}", b);
    println!("c是{}", c);
    println!("d是{}", d);

    // array
    // 长度固定声明后无法修改 只能放同一种类型的数据
    // 使用[]标识

    // 类型推断
    let a = [1, 2, 3];
    // 显式赋值
    let z: [i64; 3] = [1, 2, 3];
    println!("下标获取{}", z[1]);
    // 赋值相同的初始值
    let x = [3; 5]; // [3,3,3,3,3]
    for item in x {
        println!("{}", item);
    }
    // 解构
    let [b, c, d] = a;
    println!("b是{}", b);
    println!("c是{}", c);
    println!("d是{}", d);
}
