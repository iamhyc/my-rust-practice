
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

pub fn test(_val:i32){
    let _a = GLOBAL_VAR;
    let _a = outmost_mod::OUTMOST_VAR;
    let _a = outmost_mod::inner_mod::INNER_VAR;
    println!("test_mod test complete.");
}