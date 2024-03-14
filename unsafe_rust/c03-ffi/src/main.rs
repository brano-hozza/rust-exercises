use std::ffi::CString;

extern crate libc;

extern "C" {
    fn puts(s: *const libc::c_char) -> libc::c_int;
}

fn main() {
    let safe_hello_world = CString::new("Hello, world!").expect("CString::new failed");
    unsafe {
        puts(safe_hello_world.as_ptr());
    }
}
