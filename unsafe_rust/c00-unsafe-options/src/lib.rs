pub unsafe trait UnsafeFoo {
    fn unsafe_foo(&self);
}

pub unsafe fn unsafe_fun() {
    println!("Unsafe fun");
}

struct Bar;

// impl UnsafeFoo for Bar {
//     fn unsafe_foo(&self) {
//         println!("Unsafe foo");
//     }
// }
