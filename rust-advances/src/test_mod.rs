
//some global things accessed via "Self"
const GLOBAL_VAR:i32 = 1000;

pub mod outmost_mod {
    pub const OUTMOST_VAR:i32 = 100;

    pub mod inner_mod {
        pub const INNER_VAR:i32 = super::OUTMOST_VAR / 10;
    }

    mod test {
        // let mut var = 2;

    }

}

/** compile and run only when run `cargo test` */
#[cfg(test)]
pub mod vector_mod {
    #[test]
    pub fn test() {
        let mut v:Vec<String> = Vec::new();
        v.push( String::from("hello") );
        v.push( String::from("world") );
        let _t = &v[1];
        if let Some(val) = v.get(1) { //the best practice (P.S. "get" is immutable)
            println!("vector_mod test get 2-nd: {}, {}", val, _t);
        }
        for i in &mut v { //v.iter()
            *i += "aaa";          // *i is necessary
            (*i).push_str("bbb"); //*i is not necessary
        }
        println!("vector_mod test pass.");
    }
    //P.S. a `String` is a wrapper over a `Vec<u8>`.
}

#[cfg(test)]
pub mod traits_mod {
    pub trait SayWow {
        fn print_wow(&self){ //one trait item to impl.
            println!("Wow~~~"); //the default implementation
        }
    }
    
    pub fn is_wow_struct_raw<T:SayWow + std::fmt::Debug>(_item: &T) -> bool {
        true //compile-time check
    }
    pub fn is_wow_struct(_item:&(impl SayWow + std::fmt::Debug)) -> bool { //syntax sugar
        true //compile-time check
    }

    #[derive(Debug)]
    pub struct SampleStruct<T> {
        data: [T;2],
    }

    pub fn build_struct<T>(data: [T;2]) -> SampleStruct<T> {
        let _sample = SampleStruct{data:[1,2]};
        return SampleStruct{data};
    }

    impl<T> SampleStruct<T> {
        fn get(&self, idx:usize) -> Option<&T> {
            self.data.get(idx)
        }
    }

    impl<T : std::fmt::Debug> SayWow for SampleStruct<T> {
        fn print_wow(&self) {
            println!("Wow: {:?}", self);
        }
    }
    
    #[test]
    pub fn test() {
        let sample = build_struct([0,1]);
        is_wow_struct(&sample);
        is_wow_struct_raw(&sample);
        sample.get(0);
        sample.print_wow();
        println!("traits_mod test pass.");
    }
}

#[cfg(test)]
pub mod lifetime_mod {
    
    struct SelfishStruct<'a> {
        _hidden: &'a str
    }

    impl<'a> SelfishStruct<'a> {
        fn get(&self, ss:&'a str) {
            if ss == self._hidden {
                println!("you did it!");
            }
        }
    }

    #[test]
    pub fn test() {
        let _tmp = SelfishStruct {_hidden:"test"}; //"test" is with <'static> lifetime
        _tmp.get("test");
        println!("lifetime_mod test pass.");
    }
}

pub fn test(_val:i32){
    let _a = GLOBAL_VAR;
    let _a = outmost_mod::OUTMOST_VAR;
    let _a = outmost_mod::inner_mod::INNER_VAR;
    println!("test_mod test complete.");
}