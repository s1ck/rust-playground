use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // thread_spawn();
    // move_semantics();
    channels();
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
