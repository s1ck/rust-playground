use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};


fn main() {
    // thread_spawn();
    // move_semantics();
    // channels();
    // mutex_single_thread();
    mutex_multi_thread();
}

fn thread_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();
}

fn move_semantics() {
    let v = vec![1, 3, 3, 7];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channels() {
    let (tx, rx) = mpsc::channel();

    // mpsc -> multiple producers
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_multi_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        main()
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
