fn main() {
    // Raw pointers
    let mut x = 5;
    println!("x is: {}", x);
    let raw = &mut x as *mut i32;

    // Unsafe fun
    unsafe {
        println!("raw points at: {}", *raw);
    }

    // Lets edit values
    unsafe {
        *raw = 10;
    }

    println!("x is now: {}", x);
}
