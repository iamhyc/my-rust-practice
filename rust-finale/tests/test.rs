
mod concurrency_test {
    // use super::*;
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

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

mod oop_test {
    trait Shared {
        fn abstract_action(&self);
    }

    struct HomogeneousObjects<T:Shared> {
        members: Vec<T>                 //static dispatch
    }
    impl<T> HomogeneousObjects<T>
    where T:Shared {
        fn do_action(&self) {
            for item in self.members.iter() {
                item.abstract_action();
            }
        }
    }
    
    struct HeterogeneousObjects {
        members: Vec<Box<dyn Shared>>   //dynamic dispatch
    }
    impl HeterogeneousObjects {
        fn do_action(&self) {
            for item in self.members.iter() {
                item.abstract_action();
            }
        }
    }

    struct ObjectA;
    struct ObjectB;
    impl Shared for ObjectA {
        fn abstract_action(&self){ println!("I am Object A.") }
    }
    impl Shared for ObjectB {
        fn abstract_action(&self){ println!("I am Object B.") }
    }

    #[test]
    fn test_impl() {
        let _tmp = HomogeneousObjects {
            members : vec![ObjectA{}, ObjectA{}]
        };
        _tmp.do_action();
        
        let _tmp = HeterogeneousObjects {
            members : vec![
                Box::new(ObjectA{}),
                Box::new(ObjectB{})
            ]
        };
        _tmp.do_action();
    }
}