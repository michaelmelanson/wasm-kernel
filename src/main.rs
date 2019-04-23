#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc)]

use log::info;
extern crate alloc;

use uefi::{
    Handle,
    prelude::{Boot, SystemTable, Status, ResultExt}
};

mod wasm;
mod fs;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?.expect("UEFI services");

    let stdout = system_table.stdout();
    stdout.clear().expect_success("could not clear console");
    
    let mut fs = fs::Filesystem::new(system_table.boot_services()).expect("open filesystem");
    let init_binary = fs.load("\\bin\\init.wasm").expect("load init binary");
    info!("Init binary size: {}", init_binary.len());

    wasm::exec(init_binary);

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
