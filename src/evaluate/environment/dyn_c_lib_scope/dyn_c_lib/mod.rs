mod dyn_c_function;
pub use dyn_c_function::DynCFunction;

use std::{
    ffi::{ CStr, OsStr, c_void },
    mem::MaybeUninit,
    ops::Drop,
    os::{
        raw::{ c_char, c_int},
        unix::ffi::OsStrExt,
    },
    rc::Rc,
};

pub type DynCSym = *mut ();

#[derive(Debug)]
pub struct LibPtr(pub *mut c_void);

impl Drop for LibPtr {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.0);
        }
    }
}

#[derive(Debug)]
pub struct DynCLib {
    lib_ptr: Rc<LibPtr>,
}


impl DynCLib {
    pub fn dlopen(path: &str) -> Result<DynCLib, Error> {
        let mut v = Vec::new();
        let path = str_to_c_str(path, &mut v);

        let handle = unsafe {
            dlopen(path.as_ptr(), RTLD_LOCAL | RTLD_LAZY)
        };

        if handle.is_null() {
            let ret = unsafe {
                let msg = dlerror();
                let msg = CStr::from_ptr(msg);
                let msg = String::from(msg.to_str().unwrap());

                Error::Load(msg)
            };

            Err(ret)
        } else {
            let ret = DynCLib {
                lib_ptr: Rc::new(LibPtr(handle)),
            };

            Ok(ret)
        }
    }

    pub fn get_fn(&self, name: &str) -> Result<DynCFunction, Error> {
        let sym = self.get_sym(name)?;

        let typ = unsafe {
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

            (*extra_info).st_info
        };

        match typ {
            STT_FUNC => Ok(DynCFunction::new(sym, Rc::clone(&self.lib_ptr))),
            _ => Err(Error::NotAFunction),
        }
    }

    fn get_sym(&self, name: &str) -> Result<DynCSym, Error> {
        let mut v = Vec::new();
        let name = str_to_c_str(name, &mut v);

        let sym = unsafe {
            dlsym(self.lib_ptr.0, name.as_ptr())
        };

        if sym.is_null() {
            let ret = unsafe {
                let msg = dlerror();
                let msg = CStr::from_ptr(msg);
                let msg = String::from(msg.to_str().unwrap());

                Error::GetSym(msg)
            };

            Err(ret)
        } else {
            Ok(sym as DynCSym)
        }
    }
}

pub enum Error {
    Load(String),
    GetSym(String),
    NotAFunction,
    NoSymForSharedObjAddr,
    NoSharedObjForAddr,
}

fn str_to_c_str<'a>(s: &'a str, v: &'a mut Vec<u8>) -> &'a CStr {
    let path = OsStr::new(s);

    unsafe {
        if path.len() > 0 && path.as_bytes()[path.len() - 1] == 0 {
            CStr::from_bytes_with_nul_unchecked(path.as_bytes())
        } else {
            v.extend_from_slice(path.as_bytes());
            v.push(0);
            CStr::from_bytes_with_nul_unchecked(v.as_slice())
        }
    }
}

type Elf64Word = u32;
type Elf64Section = u16;
type Elf64Addr = u64;
type Elf64Xword = u64;

#[repr(C)]
struct DlInfo {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}

#[repr(C)]
struct ExtraInfo {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Section,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}

#[link(name = "c")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *mut c_char;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dladdr1(
        addr: *const c_void,
        info: *mut DlInfo,
        extra_info: *mut *mut ExtraInfo,
        flags: c_int
    ) -> c_int;
}

const RTLD_LOCAL: c_int = 0;
const RTLD_LAZY: c_int = 1;

const RTLD_DL_SYMENT: c_int = 1;

const STT_NOTYPE: u8 = 0;
const STT_OBJECT: u8 = 1;
const STT_FUNC: u8 = 2;
const STT_SECTION: u8 = 3;
const STT_FILE: u8 = 4;
const STT_COMMON: u8 = 5;
const STT_TLS: u8 = 6;
const STT_GNU_IFUNC: u8 = 10;
