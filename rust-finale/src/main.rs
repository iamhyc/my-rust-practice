

fn use_closure<T>(func:T)->i32
    where T: Fn(i32) -> i32
{
    func(10)
}

fn main() {
    let _tmp = 32;

    //the type of "a:i32" and return is inferred from usage
    let _closure = |a| {
        a + _tmp
    };

    println!( "Hello, world! {}", use_closure(_closure) );
}
