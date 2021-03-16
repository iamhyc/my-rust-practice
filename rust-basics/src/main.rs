
use std::io::{self,};

fn test(x:i32, y:i32) -> i32 {
    return x + y;
}

fn main() {
    // basic IO
    let mut tmp = String::new();
    io::stdin().read_line( &mut tmp ).expect("Oops.");
    println!("this trash is \"{}\".", tmp.trim());

    // basic shadowing
    let a = 5;
    let a = "hello";
    println!("shadowing you: {}!", a);

    // character type
    let c = '好';
    println!("没问题：{}！", c);

    //tuple and array
    let mut a = (2,3,"heterogeneous");
    let b = [1,2,3];
    a.2 = "{can do!}";
    println!("indexing a[2]: {}; indexing b[2]: {}", a.2, b[2]);

    //call a function
    println!("test: 1+2={}", test(1, 2));

    // block expression and simple ownership
    let x = 2;
    let y = {
        let x = x*5;
        x + 2
    };
    println!("x={} is not shadowed (y={}).", x, y);

    //try loops
    let mut number:i32 = 3;
    while number != 0 {
        number -= 1;
        if number==0 {
            println!("finally...");
        }
    }
    for item in [1,2,3,4,5].iter() {
        print!("{}, ", item);
    }
    println!();
    // std::io::stdout().flush().unwrap();
}
