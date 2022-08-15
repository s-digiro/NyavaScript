mod dyn_c_type;
pub use dyn_c_type::*;

#[cfg(test)]
mod test;

use std::{
    ffi::{ CStr, OsStr, c_void },
    ops::Drop,
    os::{
        raw::{ c_char, c_int},
        unix::ffi::OsStrExt,
    },
    rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct LibPtr(pub *mut c_void);

impl Drop for LibPtr {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.0);
        }
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn get_sym(&self, name: &str) -> Result<DynCType, Error> {
        let mut v = Vec::new();
        let cstr = str_to_c_str(name, &mut v);

        let sym = unsafe {
            dlsym(self.lib_ptr.0, cstr.as_ptr())
        };

        if sym.is_null() {
            let ret = unsafe {
                let msg = dlerror();
                let msg = CStr::from_ptr(msg);
                let msg = String::from(msg.to_str().unwrap());

                Error::GetSym(msg)
            };

            return Err(ret)
        }

        let sym = sym as *mut ();
        let typ = dyn_c_type::get_type(sym)?;
        let lib = Rc::clone(&self.lib_ptr);

        let ret = match typ {
            STT_FUNC
            | STT_GNU_IFUNC => DynCFunction::new(sym, lib).into(),
            STT_OBJECT => (sym as usize).into(),
            x => {
                let unhandled = |x: &str| Error::UnsupportedSymbolType {
                    name: name.into(),
                    typ: x.into()
                };

                let ret = match x {
                    STT_NOTYPE => unhandled("STT_NOTYPE"),
                    STT_SECTION => unhandled("STT_SECTION"),
                    STT_FILE => unhandled("STT_FILE"),
                    STT_COMMON => unhandled("STT_COMMON"),
                    STT_TLS => unhandled("STT_TLS"),
                    x => unhandled(&format!("{}", x)),
                };

                return Err(ret)
            }
        };

        Ok(ret)
    }

}

#[derive(Debug, PartialEq)]
pub enum Error {
    Load(String),
    GetSym(String),
    NotAFunction,
    NoSymForSharedObjAddr,
    NoSharedObjForAddr,
    UnsupportedSymbolType { name: String, typ: String },
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

#[link(name = "c")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *mut c_char;
    fn dlclose(handle: *mut c_void) -> c_int;
}

const RTLD_LOCAL: c_int = 0;
const RTLD_LAZY: c_int = 1;
