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


/*
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
*/


/*
//---------------------------------------------------
// match表达式

fn main() {
    let x = "1";
    let a = match x {    //  match也是一个表达式，有返回值
        "1" => 1,
        _ => 0
    };
    println!("{:?}", a);


    enum Message {
        Quit,                  
        Write(String),       
        Move {x: i32, y: i32}
    }

    let msg: Message = Message::Move {x: 10, y: 10};
    match msg {
        Message::Move{x, y} => println!("{} {}", x, y),   //  匹配枚举， 可以结构
        _ => println!("Error..."),  
    }


    let a = 5;
    match a {
        x => println!("{:?}", x),
        // _ => println!("6")      //  这一行永远不会执行到，  
    }

    match a {
        1 | 2 | 3 | 4 | 5 => println!("Got it"),   // 多重模式的匹配
        _ => println!("Error..."),
    }

    match a {
        1 ... 5 => println!("Got it"),     // 三个点号表示匹配一个范围
        _ => println!("Error..."),
    }

    let a = ' ';
    match a {
        'a' ... 'z' => println!("Got it"),
        _ => println!("Error...")
    }
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }

    let point: Point = Point {x: 10, y: 10};
    match point {
        Point {x, y} => println!("{} {}", x, y)    // 模式匹配中可以解构一个结构体
    }

    match point {
        Point {x: x1, y: y1} => println!("{} {}", x1, y1)   // 可以在解构的时候起别名
    }

    match point {
        Point {x, ..} => println!("{}", x)   // 可以使用双点号表示省略某些值
    }

    match point {
        Point {y, ..} => println!("{}", y)   // 可以使用双点号表示省略某些值
    }


    let (x, _, z) = (1, 2, 3);   // 使用下划线忽略绑定 

    #[derive(Debug)]
    enum Works {
        Item(i32, i32, i32),
        Quit,
    }

    let w = Works::Item(5, 6, 7);

    match w {
        Works::Item(..) => println!("Got it"),   //使用双点号忽略全部解构的值
        Works::Quit => println!("quit")
    }

    let x = 5;
    match x {
        ref y => println!("y is a ref of {}", y)   // match中使用ref关键字来引用
    }

    let mut x = 5;
    match x {
        ref mut y => println!("y is a mut ref of {}", y)   // 可变引用
    }

    let x = 5;
    match x {
        e @ 1 ... 5 => println!("{:?}", e),     // 用@符号绑定变量到e
        _ => println!("Error...")
    }

    match x {
        e @ 1 ... 2 | e @ 3 ... 5 => println!("{:?}", e),   // @ 和 | 一起使用， 确保每一部分都要有绑定
        _ => println!("Error...")
    }

    let w = Works::Item(5, 6, 7);
    match w {
        Works::Item(y, ..) if y > 5 => println!("y is grater than 5"),   // match 内部使用if
        Works::Item(..) => println!("ingore..."),
        Works::Quit => println!("quit..."),
    }

    let x = 5;
    let y = false;
    match x {
        4 | 5 if y => println!("Got it"),    // 使用|的时候，优先级是 （4 | 5）if y
        _ => println!("Error...")
    }

}
*/


/*
//---------------------------------------------------
// 方法（类成员函数 & 静态函数）

fn main() {
    #[derive(Debug)]
    struct Circle {
        x: f32,
        y: f32,
        r: f32
    }
 
    impl Circle {               // 给struct添加一个成员方法
        fn print(&self){
            println!("{:?}", self.x);
            println!("{:?}", self.y);
            println!("{:?}", self.r);
        }

        fn print2(&self){
            println!("{:?}", self.x);
            println!("{:?}", self.y);
            println!("{:?}", self.r);
        }

        fn staticfn(x: i32, y: i32){    // 静态方法
            println!("{} {}", x, y);
        }
    }


    let c = Circle {x: 10.0, y: 10.0, r: 5.0};
    c.print();         //调用
    Circle::staticfn(5, 6);    // 静态方法的调用

    impl Circle {
        fn set_x(&mut self, x: f32) -> &mut Circle {
            self.x = x;
            self
        }

        fn set_y(&mut self, y: f32) -> &mut Circle {
            self.y = y;
            self
        }
        
    }

    let mut c = Circle {x: 0.0, y: 0.0, r: 5.0};
    c.set_x(10.0).set_y(10.0).print();

}
*/


/*
//---------------------------------------------------
// 字符串

fn main() {
    let a: &'static str = "hello world";   // 默认类型是 &‘static str

    let mut s: String = "hello".to_string();   // 可变长字符串
    s.push_str(", world");
    println!("{:?}", s);

    let b = &s;    // String 可以强制转化为 &str
    println!("{:?}", b);

    let a = "你好";
    // println!("{:?}", a[0]);   // 编译报错, 因为字符是utf8编码的，

    for i in a.as_bytes() {
        println!("{:?}", i);    // 打印字节（6个字节）
    }

    for i in a.chars() {
        println!("{:?}", i);    // 打印字符, 这才是我们想要的遍历字符串方法
    }

    let x = a.chars().nth(0);
    match x {
        Some(a) => println!("{:?}", a),        // 访问字符串中的字符
        None => println!("Error..."),
    }

    let x = "hello world";
    let y: &str = &x[0..5];              // 获取字符串切片
    println!("{:?}", y); 

    let x = "你好";
    // let y: &str = &x[0..2];    // 编译错误，切片是字节偏移，不是字符偏移

    let a = "hello";
    let b = "world";
    // let c = a + b;    // 编译错误， 两个&str不能连接

    let c = "你好".to_string();
    let d = c + a;
    println!("{:?}", d);

    let c = "hello".to_string();
    let e = "rust".to_string();
    let f = c + &e;           // String 后面需要连接 &str
    println!("{:?}", f);      // &String 可以自动转化为 &str
}
*/ 


/*
//---------------------------------------------------
// 泛型

fn main() {

    let a: Option<i32> = Option::Some(5);  
    // let a: Option<i32> = Option::Some(5.0);  // 编译错误, 前后类型需要一致
    
    fn add1<T>(x: T, y: T) {     //泛型函数
        println!("hello");
    }

    #[derive(Debug)]
    struct Point<T> {     // 结构体泛型
        x: T,
        y: T,
    }

    
    let b = Point {x: 1.0, y: 2.0};
    let mut a = Point {x: 10, y: 20};

    impl<T> Point<T> {              
        fn swap(&mut self) {       // 内部函数不用写<T>
            std::mem::swap(&mut self.x,  &mut self.y);
        }
    }

    a.swap();
    println!("{} {}", a.x, a.y);

    let c: Vec<i32> = Vec::new();    // 在编译器推断不出来类型的时候, 手动指定数据类型
    let c = Vec::<i32>::new();       // 另外一种写法


}
*/

/*
//---------------------------------------------------
// traits

fn main() {

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        r: f64,
    }

    trait HasArea {      // 给Circle添加特性(类似于java中的接口)
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.r * self.r)
        }
    }

    let a = Circle {x: 10.0, y: 10.0, r: 10.0};
    println!("{:?}", a.area());

    // fn print_area<T>(x: T){        // 编译报错, 编译器无法推断T是那种类型, 所以就不知道area方法有没有
    //     print!("area is {}", x.area());
    // }

    fn print_area<T: HasArea>(x: T){        // 手动指定为所有实现了HasArea trait的类型
        print!("area is {}", x.area());
    }

    
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: std::cmp::PartialEq> Point<T> {    // 必须指定, 否则编译器不知道
        fn equal(&self) -> bool {
            self.x == self.y
        }
    }

    let x = Point {x: 10, y: 10};
    x.equal();


    // 比如说写一个简单的相加函数 (fuck, 语法这么复杂) 
    // 这里体现了一个rust的哲学:  尽可能的把坑消灭在编译阶段, 所以一些含糊不清的, 编译期间不能推断类型的泛型都被枪毙了
    fn add<T: std::ops::Add>(x: T, y: T) -> <T as std::ops::Add>::Output {     //泛型函数
        x + y
    }

    println!("{}", add(1, 2));

    // traits可以给任意类型添加, 这种姿势就像是给类型新加一个父类或接口

    trait NewTrait {
        fn print(&self);
    }

    impl NewTrait for i32 {
        fn print(&self) {
            println!("{:?}", self);
        }
    }

    1.print();   // 叼炸天, 可以给基本类型扩展一个方法



    fn foo<T: Clone + std::fmt::Debug>(x: T){     // 多个绑定, 同时要实现Clone和Debug两个trait, 直接用+号相加
        x.clone();
        println!("{:?}", x);   // 打印也需要指定(Debug), 否则编译不知道打印什么类型
    }

    // 上面函数也可以用where子句改写一下
    fn foo1<T>(x: T) where T: Clone + std::fmt::Debug
    {    
        x.clone();
        println!("{:?}", x);
    }


    struct X;

    trait Default {
        fn print(&self);

        fn inner_print(&self) {
            self.print();
        }
    }

    impl Default for X {
        fn print(&self) {
            println!("got it!");
        }
    }

    let a: X = X;
    a.print();
    a.inner_print();    // inner_print不用在impl块中实现, 在trait中已经有了默认处理

    trait Foo {
        fn foo(&self);
    }

    trait Foo1: Foo {     // trait的继承
        fn foo1(&self);
    }

    struct Y;

    impl Foo for Y{
        fn foo(&self) {
            print!("i am foo\n");
        }
    }

    impl Foo1 for Y {      // 实现这个的时候, Foo必须实现
        fn foo1(&self) {
            print!("i am foo1\n");  
        }
    }

    let y = Y;
    y.foo();
    y.foo1();  


    #[derive(Debug)]      // 让编译器自己实现Debug这个标准库里面的trait,   这个trait的功能是能够打印对象
    struct PointAuto {
        x: i32,
        y: i32
    }  

    print!("{:?}", PointAuto {x: 10, y: 10});


    #[derive(Debug)]
    struct Z;
    println!("{:?}", Z);

}
*/


/*
//---------------------------------------------------
// Drop trait & if let & while let

fn main() {
    struct HasDrop;

    impl Drop for HasDrop {
        fn drop(&mut self){     // 这里要求是一个 &mut类型
            println!("obj is dropped!");
        }
    }

    {
        let x = HasDrop;      // 当x被销毁时会运行drop方法
    }

    
    let option = Some(5);
    fn foo(x: i32) {
        println!("{:?}", x);
    }

    // 第一种写法
    match option {
        Some(x) => { foo(x) },
        None => {},
    }

    // 第二种写法
    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    // 第三种写法
    // 相当于: if 后面跟一个let表达式,如果解构成功就执行下面的语句,  可以这么理解, 但是if let是一个关键词
    if let Some(x) = option {      
        foo(x);
    }

    // 第一种写法
    let mut v = vec![1, 2, 3, 4, 5, 6];
    loop {
        match v.pop() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }

    //第二种写法
    let mut v = vec![1, 2, 3, 4, 5, 6];
    while let Some(x) = v.pop() {      // while let表示匹配多次
        println!("{:?}", x);
    }

}
*/



/*
//---------------------------------------------------
// 指针

fn main() {

    let reference = &4;

    match reference {
        ref val => println!("Got a value via destructuring: {:p}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;


    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:p}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    let a = &1;
    let b = &2;
    let c = &3;

    println!("{:?}", *a + *b + *c);
}
*/

/*
//---------------------------------------------------
// trait对象

fn main() {
    trait Msg {
        fn convert(&self) -> String;
    }

    impl Msg for i32 {
        fn convert(&self) -> String {
            format!("Msg is {}", *self)
        }
    }

    impl Msg for String {
        fn convert(&self) -> String {
            format!("Msg is {}", *self)
        }
    }

    let x: i32 = 5;
    println!("{}", 5.convert());

    let y: String = String::from("hello");
    println!("{}", y.convert());

    let z = Msg::convert(&x);
    println!("{}", z);

    fn do_something(x: &Msg) -> String {    // 传父类指针
        x.convert()
    }

    let a = do_something(&6 as &Msg);
    println!("{}", a);
}
*/


/*
//---------------------------------------------------
// 闭包（未完待续）

fn main() {
    let x = |x: i32| x + 1;    // 匿名函数
    println!("{:?}", x(1));

    let y = || {     // 没有参数
        1 + 1;
        2 + 2        // {} 可以写多行
    };
    println!("{:?}", y());

    let a = 5;
    let b = |x| x + a;

    println!("{:?}", b(1));

    // let c = &mut a;    // 编译错误，上面的闭包b， 实际上是引用了a（&a）, 再次mut引用会出错
    let c = &a;

    let d = vec![1,2,3,4];
    let e = || d;           //  这里d已经发生了move语义
    // println!("{:?}", d);    // 编译错误，这里访问d就会出错

    println!("------------------------------------------------------");

    let m = 1;
    let mut a: u8 = 5;
    let c = m + a;
    let d = &mut a;
    
    let mut a: u8 = 5;
    let c = move |x: u8| x + a;   // move 是
    let d = &mut a;

} */


/*
//---------------------------------------------------
// 模块

mod service;

fn main() {
    println!("{}", service::add(1, 2))
}
*/



/*
#[macro_use] 
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

use reqwest::Error;
use reqwest::header::ContentType;


#[derive(Deserialize, Debug)]
struct Name {
    name: String
}

#[derive(Deserialize, Debug)]
struct Nodes {
    cluster_name: String,
    nodes: std::collections::HashMap<String, Name>
}


fn get_all_nodes(uri: &str, auth: (&str, &str)) -> Result<Vec<String>, Error> {
    let client = reqwest::Client::new();
    let url: String = format!("{}/_cat/nodes?v", uri);
    let res: Vec<Name> = client.get(&url).header(ContentType::json()).basic_auth(auth.0, Some(auth.1)).send()?.json()?;
    let mut nodes: Vec<String> = Vec::new();
    for name in &res {
        nodes.push((&name.name).to_string())
    }
    Ok(nodes)
}


fn get_current_nodes(uri: &str, auth: (&str, &str)) -> Result<Vec<String>, Error> {
    let client = reqwest::Client::new();
    let url: String = format!("{}/_nodes/stats", uri);
    let res: Nodes = client.get(&url).header(ContentType::json()).basic_auth(auth.0, Some(auth.1)).send()?.json()?;

    let mut nodes: Vec<String> = Vec::new();
    for (_id, name) in &res.nodes {
        nodes.push((&name.name).to_string());
    }
    Ok(nodes)
}


fn consistency_check(url: &str, auth: (&str, &str)) -> Result<Vec<String>, Error> {
    let all_nodes = match get_all_nodes(url, auth) {
        Ok(nodes) => nodes,
        Err(_) => panic!()
    };

    let current_nodes = match get_current_nodes(url, auth) {
        Ok(nodes) => nodes,
        Err(_) => panic!()
    };

    let mut ret: Vec<String> = Vec::new();
    for j in &all_nodes {
        let mut flag = false;
        for k in &current_nodes {
            if j == k {
                flag = true;
                break;
            }
        }

        if !flag {
            ret.push(j.to_string());
        }
    }

    Ok(ret)
}


fn main(){
    let ret = consistency_check("", ("", "")).unwrap();
    println!("{:?}", ret);
}
*/


/*
// -------------------------------------- “==重载”  ----------------------------
fn main() {

    #[derive(Debug)]
    struct Name {
        name: String
    }

    impl PartialEq for Name {
        fn eq(&self, other: &Name) -> bool {
            true
        }
    }


    let a = Name {name: "hello".to_string()};

    let b = Name {name: "world".to_string()};

    println!("{:?}", a == b);
    
}
*/

/*
fn main() {

    // panic的写法
    // panic!("this is a {} {message}", "fancy", message = "message");

    #[derive(Debug)]
    enum Test<T, E> {
        MM(T),
        WW(E),
    }

    let a: Test<&'static str, i32> = Test::MM("22");

    println!("{:?}", a);
    
}*/



/*
fn main() {

    #[derive(Debug)]
    struct MyErr {
        desc: String
    }

    impl From<(&'static str, &'static str)> for MyErr {
        fn from((kind, detail): (&'static str, &'static str)) -> MyErr {
            println!("should be run here !");
            MyErr {desc: format!("{} : {}", kind, detail)}
        }
    }
    
    fn test() -> Result<(), MyErr> {
        return Err(::std::convert::From::from(("TestErr", "this is a error.")))
    }

    match test() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e.desc)
    }



    let a = "hello";

    for i in a.as_bytes().to_vec() {
        println!("{}", i);
    }


}
*/


/*
// ---------------字符串逆序输出----------------------------------
fn main() {
    let origin: String = "hello world".into();
    let mut res: Vec<char> = origin.chars().collect();
    res.reverse();

    let mut target: String = String::new();
    let _: Vec<_> = res.iter().map(|c| target.push(*c)).collect();
    
    println!("{:?}", target);
}*/



// -----------  统计纯英文文本中每一个字母出现的次数 ------------------------
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn process() -> std::io::Result<()> { 

    let file = File::open("/Users/samir/Desktop/Children of the Dawn.txt")?;
    let reader = BufReader::new(file);
    let mut counter_map: HashMap<char, u64> = HashMap::new();
    
    let mut lines = reader.lines().map(|l| l.unwrap());
    loop {
        
        let line: Option<String> = lines.next();
        if line.is_none() {
            break;
        }

        for c in line.unwrap().chars() {
            let c = c.to_lowercase().collect::<Vec<_>>()[0];
            if c.is_alphabetic() {
                counter_map.entry(c).or_insert(0u64);
                *(counter_map.get_mut(&c).unwrap()) += 1;
            }
        }
    }

    for (key, val) in counter_map.iter() {
        println!("{} : {}", key, val);
    }
    
    Ok(())
}


fn main() {
    let _ = process().unwrap();
}












































