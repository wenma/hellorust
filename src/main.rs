
// rust 是一个面向表达式的语言


// fn main() {
//     // 变量绑定
//     let x = 5;
//     println!("{}", x);

//     let y:i32 = 7;
//     println!("{}", y);

//     assert_eq!(y, 7);
//     println!("Hello, world!");

//     let (a, b): (bool,bool) = (true, false);
//     println!("{:?} {:?}", a, b);

//     let c: &str = "qqqqqqqqqq";
//     let d: String = c.to_string();
//     println!("{:?}{:?}", c, d);

// }

extern crate rand;    // 使用外部包

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











