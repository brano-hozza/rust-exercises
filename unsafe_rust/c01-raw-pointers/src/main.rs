use std::ptr::NonNull;

fn test_function() -> i32 {
    10
}

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

    // Create pointer to function
    let fnptr = test_function;
    let fn_addr = fnptr as usize;

    // (*fn_addr)();

    let new_ptr: fn() -> i32 = unsafe { std::mem::transmute(fn_addr as *const ()) };
    new_ptr();
    println!("Yepee pointer call works");

    // let new_ptr: fn() -> i32 = unsafe { std::mem::transmute(std::ptr::null() as *const ()) };
    // new_ptr();

    // println!("Second call fails, because UB!");

    // Lets play with NonNull
    let mut x = 13;

    let addr = &mut x as *mut i32;

    let res = NonNull::new(addr);
    match res {
        Some(val) => println!("Greate we have: {:?}", unsafe { *val.as_ptr() }),
        None => panic!("No way"),
    }

    // let x = NonNull::new(std::ptr::null_mut::<i32>()).expect("Please fail");
}
