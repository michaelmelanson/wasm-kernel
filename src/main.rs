#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc)]

extern crate alloc;

use alloc::vec;

use uefi::{
    Handle,
    prelude::{Boot, SystemTable, Status}
};
use log::info;

mod wasm;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?.expect("UEFI services");

    let stdout = system_table.stdout();
    stdout.clear();
    
    wasm::exec_init();

    //system_table.exit_boot_services();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
