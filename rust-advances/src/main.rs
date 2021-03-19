
use std::fmt;

mod test_mod;            // firstly, declare the usage of mod file/folder
use crate::test_mod::*;  // secondly, import the needed

struct Integer(i32);
// impl for struct Integer (with std::fmt interface)
impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn test_ownership(str: String) -> String {
    let mut new_str = str.clone();
    new_str.push_str("_real");
    println!("{} is taken from main.", str);
    return new_str; //ownership of "&mut String" moved out to "&String"
    //(non-sense but interesting...)
}

fn test_mut_literal(str: &str) {
    // *str = "Bazinga!";
    println!("nothing serious happened. {}", str);
}

#[derive(Debug)]
struct Test {
    int: i32,
    str: String,
}

impl Test {
    fn test(&self) {
        println!("This is a Test.test function");
    }
    fn associated() {
        ("This is a Test::associated function");
    }
}


/** doc line? okay, this is the main function. */
fn main() {
    // definition of scope
    {
        let s:&str = "test";
        println!("test scope with: \"{}\"",s);
    }

    // &str and String
    let mut s = String::from("s: &str");
    s.push_str("string: &str");
    let ss = s; //the owner is unique on heap (so "s" is invalid now)
    let ss = test_ownership(ss);
    println!("test move: {}", ss);
    
    // reference and borrowing
    /*
     * when using borrowing (copy/drop trait), the assigned variable keeps "mut" attribute;
     * when using reference, reference itself (i.e., the pointer) is borrowed;
     * Moreover, "&var" is always okay, "&mut var" only okay with one-time borrowing (::sad)
    */
    let s = "literal_string"; // non-sense using mut
    test_mut_literal(&s);     // non-sense using mut
    let tmp = &s[ .. s.find("_").unwrap() ];
    println!("slice reference by split: {}", tmp);
    let tmp:[i32; 3] = [1,2,3];
    println!("display for slice 0-th element: {}", &tmp[..][0]); 

    // struct and impl
    let test = Test{int:1, str:String::from("A")};
    let mut test1 = Test {..test}; // (bad practice) this is a borrow action
    test1.str.push_str("string: &str");
    println!("display that struct instance move (int {} remains): {:?}", test.int, test1);
    test1.test(); //`test.test()` not working for value "partially move"
    Test::associated();

    // enums and pattern matching
    let x = Option::Some("Some");
    match x {
        Option::Some(val) => println!("match some: {}", val),
        // Option::None => println!("None!"),
        _ => () //funny, very different from Python
    }
    if let Some(val) = x { // (bad practice) syntax sugar
        println!("(if let) match some: {}", val);
        println!("(if let) {:?}, x is not borrowed.", x);
    }

    // test mod
    test_mod::test( outmost_mod::OUTMOST_VAR );
}
