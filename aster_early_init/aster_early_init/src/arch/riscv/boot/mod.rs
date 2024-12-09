// SPDX-License-Identifier: MPL-2.0

//! The RISC-V boot module defines the entrypoints of Asterinas.

//pub mod smp;

//use alloc::{string::String, vec::Vec};
use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

/*
fn init_bootloader_name(bootloader_name: &'static Once<String>) {
    bootloader_name.call_once(|| "Unknown".into());
}

fn init_kernel_commandline(kernel_cmdline: &'static Once<KCmdlineArg>) {
    let bootargs = DEVICE_TREE.get().unwrap().chosen().bootargs().unwrap_or("");
    kernel_cmdline.call_once(|| bootargs.into());
}

fn init_initramfs(initramfs: &'static Once<&'static [u8]>) {
    let Some((start, end)) = parse_initramfs_range() else {
        return;
    };

    let base_va = paddr_to_vaddr(start);
    let length = end - start;
    initramfs.call_once(|| unsafe { core::slice::from_raw_parts(base_va as *const u8, length) });
}

fn init_acpi_arg(acpi: &'static Once<BootloaderAcpiArg>) {
    acpi.call_once(|| BootloaderAcpiArg::NotProvided);
}

fn init_framebuffer_info(_framebuffer_arg: &'static Once<BootloaderFramebufferArg>) {}

fn init_memory_regions(memory_regions: &'static Once<Vec<MemoryRegion>>) {
    let mut regions = Vec::<MemoryRegion>::new();

    for region in DEVICE_TREE.get().unwrap().memory().regions() {
        if region.size.unwrap_or(0) > 0 {
            regions.push(MemoryRegion::new(
                region.starting_address as usize,
                region.size.unwrap(),
                MemoryRegionType::Usable,
            ));
        }
    }

    if let Some(node) = DEVICE_TREE.get().unwrap().find_node("/reserved-memory") {
        for child in node.children() {
            if let Some(reg_iter) = child.reg() {
                for region in reg_iter {
                    regions.push(MemoryRegion::new(
                        region.starting_address as usize,
                        region.size.unwrap(),
                        MemoryRegionType::Reserved,
                    ));
                }
            }
        }
    }

    // Add the kernel region.
    regions.push(MemoryRegion::kernel());

    // Add the initramfs region.
    if let Some((start, end)) = parse_initramfs_range() {
        regions.push(MemoryRegion::new(
            start,
            end - start,
            MemoryRegionType::Module,
        ));
    }

    memory_regions.call_once(|| non_overlapping_regions_from(regions.as_ref()));
}

fn parse_initramfs_range() -> Option<(usize, usize)> {
    let chosen = DEVICE_TREE.get().unwrap().find_node("/chosen").unwrap();
    let initrd_start = chosen.property("linux,initrd-start")?.as_usize()?;
    let initrd_end = chosen.property("linux,initrd-end")?.as_usize()?;
    Some((initrd_start, initrd_end))
}
*/
