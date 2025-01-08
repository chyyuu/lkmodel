//! Startup process for monolithic kernel.

#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod trap;

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use core::panic::PanicInfo;

/// The main entry point for monolithic kernel startup.
#[cfg_attr(not(test), no_mangle)]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    info!("[rt_tour_5_3]: ...");
    init(cpu_id, dtb);
    start(cpu_id, dtb);
    info!("[rt_tour_5_3]: ok!");
    panic!("Never reach here!");
}

pub fn init(cpu_id: usize, dtb: usize) {
    axlog2::init("debug");
    exec::init(cpu_id, dtb);

    // Setup simplest trap framework.
    trap::init();
}

pub fn start(_cpu_id: usize, _dtb: usize) {
    let filename = "/sbin/init";

    let argv_init: Vec<String> = vec![filename.into()];
    let envp_init: Vec<String> = vec!["HOME=/".into(), "TERM=linux".into()];
    let _ = exec::kernel_execve(filename, argv_init, envp_init);

    let sp = task::current().pt_regs_addr();
    axhal::arch::ret_from_fork(sp);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    axhal::misc::terminate();
    #[allow(unreachable_code)]
    arch_boot::panic(info)
}
