
use std::fmt;

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
    
    // reference or borrowing
    /*
     * when using borrowing (copy/drop trait), the assigned variable keeps "mut" attribute;
     * when using reference, reference itself (i.e., the pointer) is borrowed;
     * Moreover, "&var" is always okay, "&mut var" only okay with one-time borrowing (::sad)
    */
    let s = "literal_string";
    test_mut_literal(&s);

}
