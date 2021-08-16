use std::mem::size_of;

const WIDTH: usize = size_of::<&()>();
const DOUBLE_WIDTH: usize = 2 * WIDTH;

trait Trait {
    fn print(&self);
}

struct Struct;
struct Struct2;

impl Trait for Struct {
    fn print(&self) {
        println!("struct");
    }
}

impl Trait for Struct2 {
    fn print(&self) {
        println!("struct2");
    }
}

fn print_struct(s: &Struct) {
    // always prints "struct"
    // this is known at compile-time
    s.print();
    // single-width pointer
    assert_eq!(WIDTH, size_of::<&Struct>());
}

fn print_struct2(s2: &Struct2) {
    // always prints "struct2"
    // this is known at compile-time
    s2.print();
    // single-width pointer
    assert_eq!(WIDTH, size_of::<&Struct2>());
}

fn print_trait(t: &dyn Trait) {
    // print "struct" or "struct2" ?
    // this is unknown at compile-time
    t.print();
    // Rust has to check the pointer at run-time
    // to figure out whether to use Struct's
    // or Struct2's implementation of "print"
    // so the pointer has to be double-width
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn Trait>());
}

#[test]
fn main() {
    // single-width pointer to data
    let s = &Struct;
    print_struct(s); // prints "struct"

    // single-width pointer to data
    let s2 = &Struct2;
    print_struct2(s2); // prints "struct2"

    // unsized coercion from Struct to dyn Trait
    // double-width pointer to point to data AND Struct's vtable
    print_trait(s as &dyn Trait); // prints "struct"

    // unsized coercion from Struct2 to dyn Trait
    // double-width pointer to point to data AND Struct2's vtable
    print_trait(s2 as &dyn Trait); // prints "struct2"
}

mod debug {
    use std::fmt::Debug;

    fn dbg<T: Debug + ?Sized>(t: &T) {
        // T: Debug + Sized
        println!("{:?}", t);
    }

    #[test]
    fn main() {
        let s = "my str";
        dbg(s); // &T = &str, T = str, str: Debug + !Sized ❌
    }
}

mod coercion {
    trait Trait {
        fn method(&self) {}
    }

    impl Trait for str {
        // can now call "method" on
        // 1) str or
        // 2) String since String: Deref<Target = str>
    }
    impl<T> Trait for [T] {
        // can now call "method" on
        // 1) any &[T]
        // 2) any U where U: Deref<Target = [T]>, e.g. Vec<T>
        // 3) [T; N] for any N, since [T; N]: Unsize<[T]>
    }

    fn str_fun(s: &str) {}
    fn slice_fun<T>(s: &[T]) {}

    #[test]
    fn main() {
        let str_slice: &str = "str slice";
        let string: String = "string".to_owned();

        // function args
        str_fun(str_slice);
        str_fun(&string); // deref coercion

        // method calls
        str_slice.method();
        string.method(); // deref coercion

        let slice: &[i32] = &[1];
        let three_array: [i32; 3] = [1, 2, 3];
        let five_array: [i32; 5] = [1, 2, 3, 4, 5];
        let vec: Vec<i32> = vec![1];

        // function args
        slice_fun(slice);
        slice_fun(&vec); // deref coercion
        slice_fun(&three_array); // unsized coercion
        slice_fun(&five_array); // unsized coercion

        // method calls
        slice.method();
        vec.method(); // deref coercion
        three_array.method(); // unsized coercion
        five_array.method(); // unsized coercion
    }
}

mod trait_objects {
    trait Trait {}

    // the above is REQUIRED for

    // impl Trait for dyn Trait {
    //     // compiler magic here
    // }

    // since `dyn Trait` is unsized

    // and now we can use `dyn Trait` in our program

    fn function(t: &dyn Trait) {} // compiles
}
