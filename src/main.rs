#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc)]

extern crate alloc;

use uefi::{
    Handle,
    prelude::{Boot, SystemTable, Status, ResultExt}
};

mod wasm;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?.expect("UEFI services");

    let stdout = system_table.stdout();
    stdout.clear().expect_success("could not clear console");
    
    wasm::exec_init();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
