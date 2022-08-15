use std::{
    convert::From,
    ffi::c_void,
    mem::MaybeUninit,
    os::raw::{ c_char, c_int},
};
use super::Error;

mod func;
pub use func::DynCFunction;

#[derive(Debug)]
pub enum DynCType {
    Func(DynCFunction),
    USize(usize),
}

impl From<DynCFunction> for DynCType {
    fn from(x: DynCFunction) -> DynCType {
        DynCType::Func(x)
    }
}

impl From<usize> for DynCType {
    fn from(x: usize) -> DynCType {
        DynCType::USize(x)
    }
}

pub const STT_NOTYPE: u8 = 0;
pub const STT_OBJECT: u8 = 1;
pub const STT_FUNC: u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE: u8 = 4;
pub const STT_COMMON: u8 = 5;
pub const STT_TLS: u8 = 6;
pub const STT_GNU_IFUNC: u8 = 10;

pub type DynCSym = *mut ();

const RTLD_DL_SYMENT: c_int = 1;

type Elf64Word = u32;
type Elf64Section = u16;
type Elf64Addr = u64;
type Elf64Xword = u64;

#[link(name = "c")]
extern "C" {
    fn dladdr1(
        addr: *const c_void,
        info: *mut DlInfo,
        extra_info: *mut *mut ExtraInfo,
        flags: c_int
    ) -> c_int;
}

#[derive(Debug)]
#[repr(C)]
struct DlInfo {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}

#[derive(Debug)]
#[repr(C)]
struct ExtraInfo {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Section,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}


pub fn get_type(sym: DynCSym) -> Result<u8, Error> {
    unsafe {
        let mut info = MaybeUninit::<DlInfo>::zeroed();
        let mut extra_info: *mut ExtraInfo = std::ptr::null_mut();

        let res = dladdr1(
            sym as *mut c_void,
            info.as_mut_ptr(),
            &mut extra_info,
            RTLD_DL_SYMENT
        );

        let info = info.assume_init();

        if res == 0 {
            return Err(Error::NoSharedObjForAddr)
        } else if info.dli_sname.is_null() && info.dli_saddr.is_null() {
            return Err(Error::NoSymForSharedObjAddr)
        }

        let st_info = (*extra_info).st_info;
        Ok(st_info & 0xf)
    }
}
