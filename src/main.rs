// 常量 全大写下划线分割
const MAX_DEF: u32 = 9999;

fn main() {
    println!("---------常量---------");
    // 常量 全大写下划线分割
    // use
    // invalid left-hand side of assignment
    // MAX_DEF = 123;
    println!("{}", MAX_DEF);

    println!("---------不可变---------");

    // 不可变 immutable
    let mut a = 5;
    println!("{}", a);
    // if missing 'mut' will throw "cannot assign twice to immutable variable `a`"
    a = 6;
    println!("{}", a);

    println!("---------重定义---------");
    // shadowing 重定义 (可以声明同名的变量 后声明的会覆盖掉之前声明的变量)

    let b = 12;

    println!("b是{}", b);

    let b = 18;

    // line 24 shadowing line 20
    println!("b是{}", b);
    // shadowing 与 mut的区别
    // 1. mut 赋值后 变量是可变的 shadowing 还是不可修改
    // 2. shadowing 可以实现对变量的类型进行变更而mut不行
    // exp:
    let c = "1231";
    // if remove let will throw "mismatched types expected `&str`, found `usize`"
    let c = c.len();
    println!("c的长度是:{}", c);
    
}
