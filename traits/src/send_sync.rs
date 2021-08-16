use crossbeam::thread;

#[test]
fn feature() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &greeting;

    thread::scope(|scoped_thread| {
        // spawn 3 threads
        for n in 1..=3 {
            // greeting_ref copied into every thread
            scoped_thread.spawn(move |_| {
                println!("{} {}", greeting_ref, n); // prints "Hello {n}"
            });
        }
        println!("{}", greeting_ref)
        // line below could cause UB or data races but compiler rejects it
        // greeting += " world"; // ❌ cannot mutate greeting while immutable refs exist
    })
    .unwrap();

    // can mutate greeting after every thread has joined
    greeting += " world"; // ✅
    println!("{}", greeting); // prints "Hello world"
}
