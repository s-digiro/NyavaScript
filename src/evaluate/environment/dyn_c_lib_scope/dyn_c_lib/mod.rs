mod dyn_c_function;
pub use dyn_c_function::DynCFunction;

use std::{
    ffi::{ CStr, OsStr, c_void },
    ops::Drop,
    os::{
        raw::{ c_char, c_int},
        unix::ffi::OsStrExt,
    }
};

pub type DynCSym = *mut ();

#[derive(Debug)]
pub struct DynCLib {
    lib_ptr: *mut c_void,
}

impl DynCLib {
    pub fn dlopen(path: &str) -> Result<DynCLib, Error> {
        let path = str_to_c_str(path);

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
                lib_ptr: handle,
            };

            Ok(ret)
        }
    }

    pub fn get_sym(&self, name: &str) -> Result<DynCSym, String> {
        let name = str_to_c_str(name);

        let sym = unsafe {
            dlsym(self.lib_ptr, name.as_ptr())
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

impl Drop for DynCLib {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.lib_ptr);
        }
    }
}

pub enum Error {
    Load(String),
    GetSym(String),
}

fn str_to_c_str(s: &str) -> &CStr {
    let path = OsStr::new(path);

    let mut v: Vec<u8> = Vec::new();

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

#[link(name = "c")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *mut c_char;
    fn dlclose(handle: *mut c_void) -> c_int;
}

const RTLD_LOCAL: c_int = 0;
const RTLD_LAZY: c_int = 1;
