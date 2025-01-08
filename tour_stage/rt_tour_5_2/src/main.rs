//! Startup process for monolithic kernel.

#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use alloc::vec;

/// The main entry point for monolithic kernel startup.
#[cfg_attr(not(test), no_mangle)]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    info!("[rt_tour_5_2]: ...");
    init(cpu_id, dtb);
    start(cpu_id, dtb);
    info!("[rt_tour_5_2]: ok!");
    axhal::misc::terminate();
}

pub fn init(cpu_id: usize, dtb: usize) {
    axlog2::init("debug");
    bprm_loader::init(cpu_id, dtb);
    axtrap::init(cpu_id, dtb);
    task::alloc_mm();
}

pub fn start(_cpu_id: usize, _dtb: usize) {
    let filename = "/sbin/init";
    let args = vec![filename.into()];
    let (entry, sp) = bprm_loader::execve(filename, 0, args, vec![]).unwrap();

    // Todo: check entry and sp for ld.so
    info!("Reach here! entry: {:#X}; sp: {:#X}", entry, sp);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    axhal::misc::terminate();
    #[allow(unreachable_code)]
    arch_boot::panic(info)
}
