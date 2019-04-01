mod console;

#[no_mangle]
pub extern fn main() {
    console::println("Hello from WASM!");
}
