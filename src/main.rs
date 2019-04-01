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


#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?.expect("UEFI services");

    info!("Hello, world!");

    let v = vec![1, 2, 3];

    info!("Vector is {:?}", v);

    //system_table.exit_boot_services();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
