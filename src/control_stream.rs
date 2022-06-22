use rand::Rng;
fn main() {
    // 控制流
    let rand_num = rand::thread_rng().gen_range(1..11);
    // if - else
    // 条件必须为bool
    // 分支叫做arm
    if rand_num >= 5 {
        println!("真");
    } else {
        println!("假")
    }

    // match 方式
    match rand_num.cmp(&5) {
        std::cmp::Ordering::Less => println!("假"),
        std::cmp::Ordering::Equal => println!("真"),
        std::cmp::Ordering::Greater => println!("真"),
    }

    //if表达式 注意块级区域没有分号才是表达式
    let a = if rand_num >= 5 {
        "这是真的"
    } else {
        "这是假的"
    };
    println!("{}", a);

    // loop while for
    // 循环

    // loop能够变成表达式 通过break后跟上表达式 类似return

    let a = loop {
        let rand_num = rand::thread_rng().gen_range(1..11);
        if rand_num == 10 {
            println!("到10");
            // 跳出loop
            break rand_num;
        } else if rand_num == 9 {
            println!("到9");
            // 跳出本次继续下次
            continue;
        }
    };

    println!("loop的结果:{}", a);

    // while 在进入循环体之前就先进行一次判断如。
    let mut rand_num = rand::thread_rng().gen_range(1..11);
    while rand_num < 15 {
        rand_num = rand_num + 1;
    }
    println!("rand_num的值为{}", rand_num);

    // for 多用于遍历数组集合
    let a = [1, 23, 4, 5, 6];

    // 注意！！ 使用iter能够直接获取到数组每个元素的引用 从而能够操作原数组，否则会赋值成副本。
    for item in a.iter() {
        println!("for循环结果{}", item)
    }

    // for 和 range 搭配能够实现大多数的while情况
    // range.rev()为反转这次的range
    for i in (1..11).rev() {
        println!("for循环结果{}", i);
    }
}
