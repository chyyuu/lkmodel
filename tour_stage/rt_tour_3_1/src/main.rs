#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use alloc::vec;
use core::panic::PanicInfo;
use driver_common::{BaseDriverOps, DeviceType};
use driver_block::{ramdisk, BlockDriverOps};

const DISK_SIZE: usize = 0x1000;    // 4K
const BLOCK_SIZE: usize = 0x200;    // 512

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_3_1]: ...");

    axalloc::init();

    let mut disk = ramdisk::RamDisk::new(0x1000);
    assert_eq!(disk.device_type(), DeviceType::Block);
    assert_eq!(disk.device_name(), "ramdisk");
    assert_eq!(disk.block_size(), BLOCK_SIZE);
    assert_eq!(disk.num_blocks() as usize, DISK_SIZE/BLOCK_SIZE);

    let block_id = 1;

    let mut buf = vec![0u8; BLOCK_SIZE];
    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] != *b"0123");

    buf[0] = b'0';
    buf[1] = b'1';
    buf[2] = b'2';
    buf[3] = b'3';

    info!("ramdisk: write data ..");
    assert!(disk.write_block(block_id, &buf).is_ok());
    assert!(disk.flush().is_ok());
    info!("ramdisk: write ok!");

    info!("ramdisk: read data ..");
    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] == *b"0123");
    info!("ramdisk: verify ok!");

    info!("[rt_tour_3_1]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
