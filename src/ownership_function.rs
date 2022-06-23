fn main(){

    let s = String::from("你好世界");

    has_move(s);

    //println!("{}",s);//错误 s因为是复杂类型 指针已经通过参数传递给了函数 所以此处s的指针已经被禁用(move)
    
    let s = 123;

    has_copy(s);
    
    println!("{}",s);//可以正常访问 s为简单数据类型 数据放在栈中 实现了copy方法 此处可以调用

    let z = String::from("nihao");
    let s1 = get_string(z);
    println!("{}",s1);
    // z 是访问不了的这里 原本z的指针已经被移动到s1上了
    // println!("{}",z)
}

fn has_move(s1:String){
    println!("{}",s1);
}

fn has_copy(s1:i32){
    println!("{}",s1);
}
fn get_string(s : String) -> String {
    s 
}