/*
*
*  马文 （samir_mawen@hotmail.com）
*
*  2017-10-28
*
*  rust基础学习
*
*/


#[allow(unused_variables)] 
#[allow(dead_code)]
#[allow(unused_mut)]

/*
//---------------------------------------------------
// 变量

fn main() {
    let x = 5;
    println!("{}", x);

    let y:i32 = 7;
    println!("{}", y);

    assert_eq!(y, 7);
    println!("Hello, world!");

    let (a, b): (bool,bool) = (true, false);  // 解构
    println!("{:?} {:?}", a, b);

    let c: &str = "hello world";
    let d: String = c.to_string();
    println!("{:?}{:?}", c, d);

} 
*/


/*
//---------------------------------------------------
// 一个猜数游戏

extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let magic = rand::thread_rng().gen_range(1, 100);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("unable to read string");

        println!("{:?}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(& magic){
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
    
}
*/


/*
//---------------------------------------------------
// 函数

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn add(x: u32, y: u32) {          // 没有返回值可不写 ->
    println!("hello world");
}

fn main() {
    println!("{}", add(1, 2))
}
*/


/*
//---------------------------------------------------
// 发散函数

fn p() -> ! {
    panic!("Error on something");
}

fn main() {
    p();
}
*/


/*
//---------------------------------------------------
// 函数指针

fn add(i: i32) -> i32 {
    i + 1
}

fn main() {
    let f: fn(i32) -> i32 = add;
    println!("{}", f(1));
}
*/


/*
//---------------------------------------------------
// 数组

fn main() {
    let a = [1, 2, 3];
    println!("{:?}", a.len());

    let b: [u8; 3] = [1, 2, 3];
    println!("{:?}", b.len());

    let c = [0; 20];
    println!("{:?}", c.len());

    let d: [&str; 3] = ["hello", "world", "rust"];
    println!("{:?}", d[2]); 
}
*/


/*
//---------------------------------------------------
// 切片

fn main() {
    let a: [u8; 4] = [1, 2, 3, 4];

    let b = &a[..];            // 取全部元素
    println!("{:?}", b.len()); 

    let b = &a[1..3];
    println!("{:?}", b.len()); 

    let b: &str = "hello";
    println!("{:?}", b.len());
}
*/


/*
//---------------------------------------------------
// 元组

fn main() {
    let a = (1, "hello");
    println!("{}", a.1);

    let a: (u8, &str) = (1, "hello");
    println!("{}", a.1);

    let (a, b, c) = (1, "hello", 1.0);
    println!("{}{}{}", a, b, c);

    let a = (1, "hello", 1.0);
    println!("{}", a.0);
    println!("{}", a.1);
    println!("{}", a.2);
}
*/


/*
//---------------------------------------------------
// 流程控制

fn main() {
    let x = 5;

    if x == 5 {
        println!("{:?}", x);
    } else if x == 6 {
        println!("{:?}", x);
    } else {
        println!("error");
    }

    let y = if x == 5 {10} else {11};
    println!("{}", y);

    loop {                    // 相当于 while true
        println!("hello");
        if x == 5 {
            break;
        }
    }

    while true {
        println!("hello");
        if x == 5 {
            break;
        }
    }

    for x in 0..5{              //  ..两个点表示范围
        println!("world");
    }

    for (index, x) in (0..5).enumerate(){
        println!("{} {}", index, x);
    }

    let lines = "hello\nworld".lines();
    for l in lines {
        println!("line: {}", l);
    }

    'outer: for i in 0..10 {             // 标签，以单引号‘开头
        'inner: for j in 0..10 {
            if i % 2 == 0 {
                println!("{:?}", i);
                continue 'outer;
            }

            if j % 2 == 0 {
                println!("{:?}", j);
                continue 'inner;
            }
        }
    }

}
*/


/*
//---------------------------------------------------
// vector

fn main() {
    let v = vec![1, 2, 3];      // 使用宏创建
    println!("{:?}", v[1]);

    let v = vec![0; 10];
    println!("{:?}", v[1]);

    let i: usize = 1;
    println!("{:?}", v[i]);

    let mut v: Vec<i32> = Vec::new();    // 标准创建
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v.get(1));
    println!("{:?}", v.get(100));

    for i in &v {                // 遍历，vec需要引用
        println!("{:?}", i);
    }

    for i in &v {
        println!("{:?}", i);
    }

    match v.get(1) {            // get取值，遇到index越界
        Some(x) => println!("{}", x),
        None => println!("not found"),
    }

}
*/ 


/*
//---------------------------------------------------
// 所有权 & 引用 & 生命周期    

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;

    println!("{}", v2[1]);
    // println!("{}", v[1]);    // 编译报错，vec的所有权已经转移到了v2  

    fn take(v: Vec<i32>) {
        println!("{}", v[0]);
    }

    let v3 = vec![1, 2, 3];
    take(v3);
    // println!("{}", v3[0]);  // 编译报错，在用作函数参数的时候同样发生了所有权移动

    fn take1(v: &Vec<i32>) {
        println!("{}", v[0]);
    }
    let v4: Vec<i32> = vec![1, 2, 3];
    take1(&v4);
    println!("{}", v4[0]);    // 使用引用传参，对象不发生所有权的转移


    let a1 = 3;
    let a2 = a1;
    println!("{} {}", a1, a2);   //正确，基础数据类型直接copy， 不发生所有权的移动

    // 可变引用
    fn push(v: & mut Vec<i32>){
        v.push(5)
    }

    let mut v5: Vec<i32> = vec![1, 2, 3];
    push(&mut v5);

    let v6 = &mut v5;
    println!("{:?}", v6[0]);

    // let v7 = &mut v5;      // 编译报错，mut引用在相同作用域下只能引用一次
    // let v7 = &v5;          // 编译报错，mut引用在相同作用域下仍然存在，非mut引用也是不行

    let v8 = vec![1, 2, 3];
    let v9 = &v8;
    let v10 = &v8;
    let v11 = &v8;            // 正确，只要v8没有存在mut引用，非mut引用可以存在多次
    println!("{} {} {}", v9[0], v10[0], v11[0]);


    let mut x: u8 = 8;
    {   
        let y = &mut x;     // y不是mut, 但是引用了一个mut的变量，这里的&有部分c++里面“地址”的意思
        *y += 1;            // *解引用访问内容，y不可变，但是*y（也就是x）可变
                            // 这里有点类似C++中const *p 和 * const p的区别
    }                       
    println!("{}", x);      // 上面两个语句需要用{}包起来，否则编译报错，这是因为对x的mut引用必须在print之前结束，否则print的时候再次引用会报错

    let mut x: u8 = 8;
    {  
        let y = &mut x;
        // y = &mut 1;          // 编译报错，y不可变， 但是*y可变
    }


    let a: &i32;
    let b = 1;
    // a = &b;                 // 编译报错，因为同一作用域以它声明的相反顺序释放资源，b会先于a被释放，再次访问a会出错


    // 下面代码块编译报错
    // life返回一个引用，但是他并不知道这个引用是和b的生命周期一样，还是和p的生命周期一样
    // 如果和b的一样， 则最后一个print可以正常工作
    // 如果和p的一样， 则最后一个print不能正常工作（v就变成了一个无效引用）
    // 这里编译器并不能判断，所以就有了歧义，这需要我们人工标识生命周期

    /*
    fn life(x: &str, y: &str) -> &str {        
        return x;
        // return y;
    }
    let a = "hello";
    let b = "world";
    let v;
    {
        let p = format!("hello {}", b);
        v = life(b, p.as_str());          
    }
    println!("{:?}", v);
    */

    // 正确， 新的life函数标注了生命周期，指定返回值和x变量的生命周期一样
    fn life<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {        
        return x;
        // return y;
    }
    let b = "world";
    let v;
    {
        let p = format!("hello {}", b);
        v = life(b, p.as_str());          
    }
    println!("{:?}", v);


    // 'static生命周期，局部静态变量
    let x: &'static str = "hello world";
    // 全局变量
    static PI: f32 = 3.1415926;

    let a: fn (a: u8, b: &str) -> &str;  
    // let a: fn<'a> (a: u8, b: &'a str) -> &'a str;  // 隐式生命周期推断 

}
*/


/*
//---------------------------------------------------
// 可变性

fn main() {
    fn name(mut x: i32) {
        unimplemented!();
    }

    // fn name1(x: mut i32) { unimplemented!(); }   //编译报错，mut在无引用的情况下应该写在x前面

    #[derive(Debug)]
    struct Point {    // 结构体的可变性是整体的可变性，与内部x, y无关
        x: u8,
        y: u8
    }

    let mut a = Point {x: 10, y: 20};
    a.x = 10;

    let b = Point {x: 10, y: 20};
    // b.x = 10;    //编译报错， b不可变
}
*/


/*
//---------------------------------------------------
// 结构体

fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
        // mut z: i32   // 字段级别不支持可变性，struct的可变性在他整体 
    }

    let a = Point {x: 100, y: 200};  //结构体的初始化
    println!("{:?}", a.x);

    let mut b = Point {x: 100, y: 200};
    b.x += 5;
    println!("{:?}", b.x);

    #[derive(Debug)]
    struct PointRef<'a> {    //内部有两个引用，所以必须手动指定生命周期
        x: &'a mut i32,      // 引用支持可变性
        y: &'a mut i32,
    }

    let mut point1 = Point {x: 1, y: 2};
    { 
        let pointref1 = PointRef {x: &mut point1.x, y: &mut point1.y};
        *pointref1.x += 5;
        *pointref1.y += 5;

    }
    println!("{} {}", point1.x, point1.y);

    #[derive(Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32
    }

    let mut point3d = Point3d {x: 0, y: 0, z: 0};
    point3d = Point3d {x: 1, ..point3d};      //  struct更新语法
    point3d = Point3d {x: 1, z: 2, ..point3d};

    println!("{} {} {}", point3d.x, point3d.y, point3d.z);


    struct Color(i32, i32, i32);    // 元组结构体，没有field name
    let color = Color(255, 254, 253);
    println!("{:?}", color.0);

    let Color(r, g, b) = color;
    println!("{} {} {}", r, g, b);   // 元组结构体的解构
    println!("{:?}", color.0);     // 解构语法不发生转移语义，color还能正常访问

    struct People {};              // 类单元结构体
    struct Person;

    let p = People {};
    let p1 = Person;
    // let p1 = Person {};  // 编译出错，初始化需要和其声明形式一样
}
*/


//---------------------------------------------------
// 枚举
fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,                  // 类单元结构体
        Write(String),         // 元组结构体
        Move {x: i32, y: i32}  // 一般结构体
    }

    let message: Message = Message::Write("hello world".to_string());
    let message: Message = Message::Move {x: 10, y: 10};  // 初始化

    // let Message::Move {x, y} = message;    // 编译报错， 不能解构一个enum

    // 含有元组结构体的枚举初始化的时候很想是一个函数调用
    // 下面是将一个Vec<String>类型转化为Vec<Message>类型
    let v: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
    for i in &v1 {
        println!("{:?}", i);
    }

}










