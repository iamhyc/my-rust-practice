
use std::thread;
use std::time::Duration;

mod concurrency_test {
    use super::*;

    #[test]
    fn test() {
        let _handler = thread::spawn(|| {
            thread::sleep(Duration::from_millis(500));
            println!("delayed response from spawned.");
        });

        println!("response from main thread.");
    }
}