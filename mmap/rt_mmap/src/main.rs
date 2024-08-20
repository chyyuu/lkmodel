#![no_std]
#![no_main]
#![feature(map_try_insert)]

use core::str::FromStr;
use core::panic::PanicInfo;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use mm::MmStruct;
use fileops::AT_FDCWD;
use alloc::string::String;
use alloc::string::ToString;
use crate::alloc::borrow::ToOwned;
use axerrno::{linux_err_from, LinuxError};

#[macro_use]
extern crate axlog2;
extern crate alloc;

type FdMap = BTreeMap<usize, usize>;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("info");
    info!("[rt_mmap]: ...");

    axhal::arch_init_early(cpu_id);
    axalloc::init();
    page_table::init();
    fileops::init(cpu_id, dtb_pa);
    task::init(cpu_id, dtb_pa);
    task::alloc_mm();

    //let lines = include_str!("/tmp/mmap_cases/0x8000100000081e92.flow");
    let lines = include_str!("/tmp/mmap_cases/0x8000400000081f2c.flow");
    do_testcase(lines);

    info!("[rt_mmap]: ok!");
    axhal::misc::terminate();
}

fn do_testcase(lines: &str) {
    let mut fd_map: FdMap = FdMap::new();
    for line in lines.split('\n') {
        //info!("line: {}", line);
        do_cmd(line, &mut fd_map);
    }
}

fn do_cmd(cmd: &str, fd_map: &mut FdMap) {
    let info: Vec<_> = cmd.split('|').collect();
    let name = info[0];
    info!("NAME: [{}]", name);

    match name {
        "openat" => do_openat(&info, fd_map),
        "close" => do_close(&info, fd_map),
        "mmap" => do_mmap(&info, fd_map),
        "munmap" => do_munmap(&info, fd_map),
        _ => {},
    }
}

fn do_openat(info: &Vec<&str>, fd_map: &mut FdMap) {
    let dfd = info[1].trim();
    let fname = remove_quotes(info[2].trim());
    let flags = parse_usize(info[3].trim());
    let mode = parse_usize(info[4].trim());
    let result = info[6].trim();
    assert_eq!(dfd, "AT_FDCWD");
    if result.starts_with("E") {
        info!("openat: {} error! Ignore it!", fname);
        return;
    }
    let fd = fileops::register_file(fileops::openat(AT_FDCWD, &fname, flags, mode));
    let ofd = parse_usize(result);
    fd_map.try_insert(ofd, fd).unwrap();
    info!("do_openat: ... {}, {}, {}, {}, ofd {:#x} => fd {:#x}", dfd, fname, flags, mode, ofd, fd);
}

fn do_close(info: &Vec<&str>, fd_map: &mut FdMap) {
    let ofd = parse_usize(info[1].trim());
    let fd = fd_map.remove(&ofd).unwrap();
    info!("close: ofd {:#x} => fd {:#x}", ofd, fd);
    fileops::unregister_file(fd);
}

//
// Format:
// mmap|NULL| 0x1000| prot 0| flags 0| 3| 0x0| | 0x3ff7ea5000| usp
//
fn do_mmap(info: &Vec<&str>, fd_map: &mut FdMap) {
    let va = parse_usize(info[1]);
    let len = parse_usize(info[2]);
    let prot = parse_usize(info[3]);
    let flags = parse_usize(info[4]);
    let ofd = parse_usize(info[5]);
    let offset = parse_usize(info[6]);
    let result = info[8].trim();
    error!("mmap: va {:#x} len {:#x} prot {:#x} flags {:#x} ofd {} off {:#x} result {}",
          va, len, prot, flags, ofd, offset, result);
    let fd = if ofd as i64 != -1 {
        fd_map.get(&ofd).unwrap()
    } else {
        &ofd
    };
    let ret = mmap::mmap(va, len, prot, flags, *fd, offset)
        .unwrap_or_else(|e| {
            linux_err_from!(e)
        });
    error!("ret {:#x}", ret);
}

//
//munmap|0x3ff7ea5000| 0x1000| | OK| usp
//
fn do_munmap(info: &Vec<&str>, fd_map: &mut FdMap) {
    let va = parse_usize(info[1]);
    let len = parse_usize(info[2]);
    info!("munmap: ...");
    mmap::munmap(va, len);
}

fn parse_usize(s: &str) -> usize {
    let s = s.trim();
    assert!(s.starts_with("0x"), "input: {}", s);
    usize::from_str_radix(&s[2..], 16).unwrap()
}

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    axhal::misc::terminate();
    arch_boot::panic(info);
}
