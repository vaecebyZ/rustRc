// use std::io; // prelud 默认全局导入

fn main() {
    // 需求:生成一个1-100的随机数 获取用户输入 大于提示大于 小于提示小于 等于推出程序
    println!("猜测一个数字:");

    // 声明一个guess变量为string类型 直接实例化一个Sting空类型赋值给guess
    let mut guess = String::new();

    // std全局调用io库里面的stdin里面的readLine函数给guess赋值 
    // buf是按照引用地址传递所以需要增加 &(取地址符号) 而且引用默认不可变需要增加mut关键字
    std::io::stdin()
        .read_line(&mut guess)
        .expect("无法读取行内数据哦");

    // 打印出输出值 | {}为占位符
    println!("您猜测的值为，{}", guess);
    
}
