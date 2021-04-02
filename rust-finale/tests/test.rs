
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod concurrency_test {
    use super::*;

    #[test]
    fn test_thread() {
        let _handler = thread::spawn(|| {
            thread::sleep(Duration::from_millis(500));
            println!("delayed response from spawned.");
        });

        println!("response from main thread.");
        _handler.join().unwrap();
    }

    #[test]
    fn test_channel() {
        let (_tx, rx) = mpsc::channel::<i32>();
        
        let tx = _tx.clone();
        thread::spawn(move || {
            for i in 100..123 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(10)); //send every 10ms
            }
        });

        println!("blocking received: {}", rx.recv().unwrap());
    }
}