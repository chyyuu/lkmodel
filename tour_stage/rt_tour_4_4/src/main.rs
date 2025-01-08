#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use wait_queue::WaitQueue;
use mutex::Mutex;
use userboot::USER_APP_ENTRY;
use axhal::mem::PAGE_SIZE_4K;
use fork::CloneFlags;
use mmap::{PROT_READ, PROT_WRITE, PROT_EXEC, MAP_FIXED};

static WQ: WaitQueue = WaitQueue::new();
static APP_READY: Mutex<bool> = Mutex::new(false);

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_4_4]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Startup a kernel thread.
    task::init(cpu_id, dtb_pa);
    trap::start();

    let tid = fork::user_mode_thread(move || {
        // Prepare for user app to startup.
        userboot::init(cpu_id, dtb_pa);

        info!("App kernel-thread load ..");
        // Load userland app into pgd.
        userboot::load();

        // Note: wait wander-thread to notify.
        info!("App kernel-thread waits for wanderer to notify ..");
        *APP_READY.lock() = true;
        WQ.wait();

        // Start userland app.
        info!("App kernel-thread is starting ..");
        userboot::start();
        userboot::cleanup();
    }, CloneFlags::CLONE_FS);
    assert_eq!(tid, 1);

    let tid = fork::kernel_thread(move || {
        info!("Wander kernel-thread is running ..");

        // Alloc new pgd and setup.
        task::alloc_mm();

        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        let _ = mmap::_mmap(USER_APP_ENTRY.into(), PAGE_SIZE_4K, prot, MAP_FIXED, None, 0);
        mmap::faultin_page(USER_APP_ENTRY, 0);
        info!("Wanderer: Map user page: {:#x} ok!", USER_APP_ENTRY);

        // Try to access user app code area
        let size = 16;
        let run_code = unsafe { core::slice::from_raw_parts_mut(USER_APP_ENTRY as *mut u8, size) };
        info!("Try to access app code: {:?}", &run_code[0..size]);

        info!("Wander kernel-thread waits for app to be ready ..");
        while !(*APP_READY.lock()) {
            run_queue::yield_now();
        }
        info!("Wander notifies app ..");
        WQ.notify_one(true);
    }, CloneFlags::CLONE_FS);
    assert_eq!(tid, 2);

    loop {
        task::yield_now();
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
