use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{self, Duration, Instant},
};

use crossbeam::select;

fn main() {
    let start = Instant::now();

    let h1 = thread::spawn(|| thread::sleep(Duration::from_millis(300)));
    let h2 = thread::spawn(|| thread::sleep(Duration::from_millis(300)));

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Took {:?}", start.elapsed());

    // cond_var();
    crossbeam_foo();
}

fn cond_var() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
}

fn move_foo() {
    let duration = Duration::from_secs(1);

    let h1 = thread::spawn(move || {
        thread::sleep(duration);
    });

    let h2 = thread::spawn(move || {
        thread::sleep(duration);
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

#[derive(Debug)]
enum Message {
    Ping,
    Pong,
    Exit,
}

fn crossbeam_foo() {
    let num_messages = 4;

    let (request_tx, request_rx) = crossbeam::channel::unbounded::<Message>();
    let (response_tx, response_rx) = crossbeam::channel::unbounded::<Message>();

    thread::spawn(move || loop {
        match request_rx.recv().unwrap() {
            Message::Ping => response_tx.send(Message::Pong).unwrap(),
            Message::Pong => eprintln!("Unexpected message"),
            Message::Exit => return,
        }
    });

    for _ in 0..num_messages {
        request_tx.send(Message::Ping).unwrap();
    }
    request_tx.send(Message::Exit).unwrap();

    for _ in 0..num_messages {
        select!(
            recv(response_rx) -> msg => println!("{:?}", msg),
        )
    }
}
