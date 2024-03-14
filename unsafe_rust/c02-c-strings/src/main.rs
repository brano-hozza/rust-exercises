use std::ffi::CString;

fn main() {
    // Lets show the difference between a string and a c-string

    let rust_string = String::from("Hop!");
    let c_string = CString::new("Hop!").expect("CString::new failed");

    // The difference is that a c-string is a null-terminated string
    // This means that the string is terminated by a null byte
    // This is useful when interfacing with C code, as C uses null-terminated strings
    // This is also why we need to use the expect method, as it will panic if the string contains a null byte

    // Lets print the strings
    println!("Rust string: {}", rust_string);
    println!("C string: {:?}", c_string);
    println!("C string as bytes: {:?}", c_string.to_bytes_with_nul());

    // Lets try to create a c-string with a null byte
    match CString::new(vec![b'H', b'o', b'p', b'!', 0]) {
        Err(e) => println!("Correct problem with trailing zero: {}", e),
        _ => panic!("Should not be ok"),
    }

    let correct = CString::new(vec![b'H', b'o', b'p', b'!']).expect("CString::new failed");
    println!("Correct C string: {:?}", correct);

    // Lets convert the c-string to a raw pointer
    let mut c_string_ptr = c_string.as_ptr() as *mut u8;

    // Change some letters in c_string
    unsafe {
        println!("Replacing 1 char ({}) with J", *c_string_ptr as char);
        *c_string_ptr = b'J' as u8;
        c_string_ptr = c_string_ptr.add(1);
        println!("Replacing 2 char ({}) with o", *c_string_ptr as char);
        *c_string_ptr = b'\0' as u8;
        c_string_ptr = c_string_ptr.add(3);
        println!("Replacing 5 char ({}) with p", *c_string_ptr as char);
        *c_string_ptr = b'p' as u8;
    }

    // Print the c-string
    println!("C string: {:?}", c_string);
    println!("C string as bytes: {:?}", c_string.to_bytes_with_nul());
    println!(
        "C string as Rust string: {:?}",
        c_string.to_str().expect("Should be ok?")
    );
}
