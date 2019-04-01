extern {
    fn console_log(ptr: *const u8, len: usize);
}

pub fn println(msg: &str) {
    unsafe {
        console_log(msg.as_ptr(), msg.len());
    }
}
