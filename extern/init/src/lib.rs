extern {
    fn console_log(ptr: *const u8, len: usize);
}

fn println(msg: &str) {
    unsafe {
        console_log(msg.as_ptr(), msg.len());
    }
}

#[no_mangle]
pub extern fn main() {
    println("Hello from WASM!");
}
