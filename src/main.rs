use std::cmp::Ordering;

fn main() {
    slice_method();
    // mut_reference_method();
    // imut_reference_method();
    // while_method();
    // loop_method();
    // for_method();
    // if_method();
    // tup_method();
    // array_method();
    // match_method();
}

//切片
fn slice_method(){
 //支持字符串字面值参数
 let a = "Hello World";
 let a1 = slice_method_01(a);

 //支持字符串类型参数
 let b = String::from("Hello World");
 let b1 = slice_method_01(&b);

 println!("空格之后的值，a1:{} b1:{}",a1,b1);
}

fn slice_method_01(s:&str)-> &str{
    let b = s.as_bytes();
    for (i,&v) in b.iter().enumerate(){
        if v == b' ' {
            return &s[i..];//获取字符串的切片
        }
    }
    return &s[..]//获取字符串的切片
}


//函数调用时，传递可变引用
fn mut_reference_method() {
    let mut str = String::from("Hello");
    let (s, len) = mut_reference_method_01(&mut str);
    println!("str:{} s:{} len:{}", str, s, len) //两个字符串的内容一样
}

fn mut_reference_method_01(s: &mut String) -> (String, usize) {
    let len = s.len();
    s.push_str(",World");
    (s.to_string(), len)
}

//函数调用时，传递不可变引用
fn imut_reference_method() {
    let str = String::from("Hello");
    let (s, len) = imut_reference_method_01(&str);
    println!("s:{} len:{}", s, len)
}

fn imut_reference_method_01(s: &String) -> (String, usize) {
    let len = s.len();
    (s.to_string(), len)
}

//如参、返回参
fn return_method() {
    let str = String::from("Hello");
    let (s, len) = return_method_01(str);
    println!("s:{} len:{}", s, len)
}

fn return_method_01(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

//while
fn while_method() {
    let mut a = 0;
    while a <= 3 {
        a += 1;
        println!("while a:{}", a);
    }
    println!("loop end");
}

//loop
fn loop_method() {
    let mut a = 0;
    loop {
        a += 1;
        println!("loop a:{}", a);
        if (a >= 3) {
            println!("loop end");
            break;
        }
    }
}

//for
fn for_method() {
    let a = [3, 4, 5, 6];
    for b in a.iter() {
        println!("data in array:{}", b);
    }
}

//if
fn if_method() {
    let a = if true { 5 } else { 3 };
    println!("number is:{}", a)
}

//元组（tup）
fn tup_method() {
    let tup: (i32, u32, f32) = (-1, 22, 33.2);
    //方式一：直接获取元组的值
    println!("直接获取元组值：{} {} {}", tup.0, tup.1, tup.2);
    //方式二：解构方式获取元组的值
    let (a, b, c) = tup;
    println!("解构方式获取元组的值{} {} {}", a, b, c);
}

//数组
fn array_method() {
    //数组
    let a = [1, 2, 3];
    println!(
        "数组长度：{},  第一个元素：{} 元素：{}",
        a.len(),
        a.first().expect("not exist"),
        a[2]
    );
}


//match
fn match_method() {
     //案例一
     let target = 50;
     let input_num = 50;
     match input_num.cmp(&target) {
         Ordering::Less => println!("猜的结果太小了"),
         Ordering::Greater => println!("猜的结果太大了"),
         Ordering::Equal => println!("你猜对了！！")
     }
 
     //案例二：
     let a = String::from("25");
     let b:u32 =  match  a.parse() {
         Ok(num) => num,
         Err(_) => 0,
     };
     println!("b:{}",b)
}