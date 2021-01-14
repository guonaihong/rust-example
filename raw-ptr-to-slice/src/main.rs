use std::fs;
use std::mem;

struct Test {
    a: i32,
}

fn main() {
    let t = Test { a: 65 };
    let view = &t as *const _ as *const u8;

    let slice = unsafe { std::slice::from_raw_parts(view, mem::size_of::<Test>()) };

    fs::write("test.txt", slice).unwrap();
}
