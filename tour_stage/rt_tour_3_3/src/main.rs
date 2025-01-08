#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_3_3]: ...");

    fstree::init(cpu_id, dtb_pa);

    let fs = fstree::init_fs();
    let locked_fs = fs.lock();

    let fname = "/sbin/origin.bin";
    let buf = axfile::api::read(fname, &locked_fs).unwrap();
    info!("read test file: {:?}; size [{}]", buf, buf.len());

    info!("[rt_tour_3_3]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
