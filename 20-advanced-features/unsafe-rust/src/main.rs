use std::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    println!("r1 is {:#?}, r2 is {:#?}", r1, r2);

    let _address = 0x12345usize;
    // let r = &address as *const i32;

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut vec![1, 2, 3]);
    assert_eq!(b, &mut vec![4, 5, 6, 7, 8]);

    println!("a is {:#?}, b is {:#?}", a, b);
}

unsafe fn dangerous() {
    println!("dangerous")
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

