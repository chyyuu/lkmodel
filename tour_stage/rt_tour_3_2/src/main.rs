#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use alloc::vec;
use core::panic::PanicInfo;
use driver_common::{BaseDriverOps, DeviceType};
use driver_block::BlockDriverOps;

const BLOCK_SIZE: usize = 0x200;    // 512

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_3_2]: ...");

    page_table::init(cpu_id, dtb_pa);

    let mut alldevs = axdriver::init_drivers2();
    let mut disk = alldevs.block.take_one().unwrap();

    assert_eq!(disk.device_type(), DeviceType::Block);
    assert_eq!(disk.device_name(), "virtio-blk");
    assert_eq!(disk.block_size(), BLOCK_SIZE);

    let block_id = 1;

    let mut buf = vec![0u8; BLOCK_SIZE];
    assert!(disk.read_block(block_id, &mut buf).is_ok());

    buf[0] = b'0';
    buf[1] = b'1';
    buf[2] = b'2';
    buf[3] = b'3';

    assert!(disk.write_block(block_id, &buf).is_ok());
    assert!(disk.flush().is_ok());

    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] == *b"0123");
    info!("virtblk: verify ok!");

    info!("[rt_tour_3_2]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
