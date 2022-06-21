// use std::io; // prelud 默认全局导入
use rand::Rng; // trait(类似接口)
use std::cmp::Ordering; //使用枚举类型
fn main() {
    // 需求:生成一个1-100的随机数 获取用户输入 大于提示大于 小于提示小于 等于推出程序

    // 声明一个随机数
    let rand_num = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是：{}", rand_num);

    loop {
        println!("猜测一个数字:");

        // 声明一个guess变量为string类型 直接实例化一个Sting空类型赋值给guess
        let mut guess = String::new();

        // std全局调用io库里面的stdin里面的readLine函数给guess赋值
        // buf是按照引用地址传递所以需要增加 &(取地址符号) 而且引用默认不可变需要增加mut关键字
        std::io::stdin()
            .read_line(&mut guess)
            .expect("无法读取行内数据哦");

        // 将目标数据转换为i32(有符号的整数)
        // match() 枚举处理异常 trim()去掉空格 parse()转换数据类型
        // 同名的变量将会被隐藏 shadow
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                // 打印错误信息
                println!("请输入正确的数字!{}",msg);
                // 跳出本次loop继续执行
                continue;
            }
        }; 

        // 打印出输出值 | {}为占位符
        println!("您猜测的值为，{}", guess);

        // 对比输入的数字与生成的数字的大小 match 表达式 cmp使用对比返回Ordering枚举
        // 接受参数为目标变量的引用
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("居然猜中了！");
                // 使用break跳出loop的循环
                break;
            }
        }
    }
}
