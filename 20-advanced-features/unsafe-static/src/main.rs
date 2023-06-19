use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-4));
    }

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &mut[i32] = unsafe {
         slice::from_raw_parts_mut(r, 1000)
    };

    println!("values is {:#?}", values);
}
