
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


/*
//---------------------------------------------------
// 切片

fn main() {
    let a: [u8; 4] = [1, 2, 3, 4];

    let b = &a[..];
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

    loop {
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

    for x in 0..5{
        println!("world");
    }

    for (index, x) in (0..5).enumerate(){
        println!("{} {}", index, x);
    }

    let lines = "hello\nworld".lines();
    for l in lines {
        println!("line: {}", l);
    }

    'outer: for i in 0..10 {
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


fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v[1]);

    let v = vec![0; 10];
    println!("{:?}", v[1]);

    let i: usize = 1;
    println!("{:?}", v[i]);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v.get(1));
    println!("{:?}", v.get(100));

    for i in &v {
        println!("{:?}", i);
    }

    for i in &v {
        println!("{:?}", i);
    }

    match v.get(1) {
        Some(x) => println!("{}", x),
        None => println!("not found"),
    }



}




