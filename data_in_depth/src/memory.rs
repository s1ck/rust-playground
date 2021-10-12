use core::ffi;
use std::{
    borrow::Cow,
    ffi::CStr,
    mem::size_of,
    os::{linux::raw, raw::c_char},
};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn print_layout() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8; 11]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} byte", size_of::<usize>());
    println!("  value:    {:?}", a);

    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} byte", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);

    println!("c (a 'box' for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} byte", size_of::<Box<[u8; 10]>>());
    println!("  points to: {:p}", c);

    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} byte", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);

    println!("C (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} byte", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
}

fn raw_ptr() {
    let a: i64 = 42;
    let a_ptr = &a as *const _;

    println!("a = {}, &a = {:p}, a_ptr = {:p}", a, &a, a_ptr);
}

fn raw_ptr_transmute() {
    let a: i64 = 42;
    let a_ptr = &a as *const _;

    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!(
        "a = {}, &a = {:p}, a_ptr = {:p}, a_addr = 0x{:x}",
        a,
        &a,
        a_ptr,
        a_addr + 7
    );
}

fn unsafe_wtf() {
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}

fn ffi() {
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        // &B as *mut u8;
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a = {}, b = {}, c = {}", a, b, c);
}

#[test]
fn run() {
    print_layout();
    raw_ptr();
    raw_ptr_transmute();
    ffi();
    unsafe_wtf();
}

struct Foo {

}

#[test]
fn feature() {
    let x= std::mem::size_of::<Foo>();

    println!("{}", x);
}
