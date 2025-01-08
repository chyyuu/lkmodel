#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;
use elf::abi::{PT_INTERP, PT_LOAD};
use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::parse::ParseAt;
use elf::segment::ProgramHeader;
use elf::segment::SegmentTable;
use axio::SeekFrom;
use axtype::PAGE_SIZE;

const ELF_HEAD_BUF_SIZE: usize = 256;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_5_1]: ...");

    fileops::init(cpu_id, dtb_pa);

    let filename = "/sbin/init";
    let file = fileops::do_open(filename, 0).unwrap();

    let mut file = file.lock();
    let mut buf: [u8; ELF_HEAD_BUF_SIZE] = [0; ELF_HEAD_BUF_SIZE];

    file.read(&mut buf).unwrap();
    let ehdr = ElfBytes::<AnyEndian>::parse_elf_header(&buf[..]).unwrap();
    info!("e_entry: {:#X}", ehdr.e_entry);

    let phnum = ehdr.e_phnum as usize;
    // Validate phentsize before trying to read the table so that we can error early for corrupted files
    let entsize = ProgramHeader::validate_entsize(ehdr.class, ehdr.e_phentsize as usize).unwrap();
    let size = entsize.checked_mul(phnum).unwrap();
    assert!(size > 0 && size <= PAGE_SIZE);
    let phoff = ehdr.e_phoff;
    //let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
    let mut buf: [u8; 2 * 1024] = [0; 2 * 1024];
    info!("phoff: {:#X}", ehdr.e_phoff);
    let _ = file.seek(SeekFrom::Start(phoff));
    file.read(&mut buf).unwrap();
    let phdrs = SegmentTable::new(ehdr.endianness, ehdr.class, &buf[..]);

    for phdr in phdrs {
        if phdr.p_type != PT_LOAD && phdr.p_type != PT_INTERP {
            continue;
        }

        info!(
            "phdr: offset: {:#X}=>{:#X} size: {:#X}=>{:#X}",
            phdr.p_offset, phdr.p_vaddr, phdr.p_filesz, phdr.p_memsz
        );
    }

    info!("[rt_tour_5_1]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
