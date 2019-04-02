use wasm_kernel_api as api;

#[no_mangle]
pub extern fn main() {
    api::console::println(format!("Hello {}!", "World").as_str());
}
