
use std::rc::Rc;
use std::cell::RefCell;

fn use_closure<T>(func:&mut T, val:i32)->i32
    where T: FnMut(i32) -> i32 //FnMut is allowed to change environment
{
    func(val)
}

fn main() {
    let mut _tmp = 32;

    // the type of "a:i32" and return is inferred from usage
    // (closure dynamically capture environment with overhead)
    let mut _closure = |a| {
        _tmp = a + _tmp;
        _tmp
    };

    println!( "Hello, {}!", use_closure(&mut _closure, 10) );
    println!( "Hello, again! {}?", use_closure(&mut _closure, 10) );

    // smart pointers test
    #[warn(dead_code)]
    struct Temp(Box<Temp>); //dead definition, should use "Option<T>"
    let _tmp = Box::new("hello");
    drop(_tmp);
    //
    let _tmp = Rc::new(RefCell::new(String::from("world"))); //Rc->RefCell->"world" (on heap)
    let _a = Rc::clone(&_tmp);
    (*(*_a).borrow_mut()).push_str("hah"); //borrow_mut on RefCell to mut
    println!("This mutable RefCell {:?}", *_a); //debug RefCell print
}
