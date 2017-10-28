
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

    let (a, b): (bool,bool) = (true, false);
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









