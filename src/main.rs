use std::ffi::c_int;

extern {
    fn add(a: c_int, b: c_int) -> c_int;
}


fn main() {
    let result = unsafe { add(1, 2) as i32 };
    println!("result: {}", result);
}
