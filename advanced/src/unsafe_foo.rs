#[cfg(test)]
mod tests {
    use std::slice::{from_raw_parts_mut};

    #[test]
    fn raw_pointers() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        let r3 = &mut num as *mut i32;

        unsafe {
            *r2 = 42;
            *r3 = 84;
        }

        assert_eq!(unsafe { *r1 }, 84);
        assert_eq!(84, num);
    }

    #[test]
    fn raw_pointer_from_address() {
        let address = 0x012345_usize;
        let r = address as *const i32;

        let x = unsafe { *r };
        println!("{}", x);
    }

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();

        assert!(mid <= len);

        // same as `slice.as_mut_ptr()`
        let p = slice as *mut [i32] as *mut i32;

        unsafe { (from_raw_parts_mut(p, mid), from_raw_parts_mut(p.add(mid), len - mid)) }
    }

    #[test]
    fn split_at_mut_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    static mut FOOBAR: &str = "foobar";

    #[test]
    fn modify_global_var() {
        unsafe {
            FOOBAR = "baz";
        }

        println!("{}", unsafe { FOOBAR });
    }
}