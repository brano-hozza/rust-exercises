//
// Unsafe implementation
//
// use c00_unsafe_options::UnsafeFoo;
//
// struct MyStruct;
//
// impl UnsafeFoo for MyStruct {
//     fn unsafe_foo(&self) {
//         println!("UnsafeFoo for i32");
//     }
// }

struct MyStruct {
    x: i32,
}

impl MyStruct {
    fn get_mut(&mut self) -> &mut Self {
        self
    }
}

fn main() {
    //
    // Call unsafe function
    //
    // unsafe_fun();

    let mut x = MyStruct { x: 1 };
    let y = x.get_mut();

    y.x = 2;

    println!("x.x = {}", x.x);
}
