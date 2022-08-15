use std::fmt::{ Display, Formatter, Result as fmtResult };
use std::rc::Rc;
use std::slice;
use super::super::{ DynCSym, LibPtr };

#[derive(Clone, Debug)]
pub struct DynCObject {
    ptr: DynCSym,
    len: u64,

    // This prevents lib ptr is in from dropping
    lib: Rc<LibPtr>,
}

impl DynCObject {
    pub fn new(ptr: DynCSym, len: u64, lib: Rc<LibPtr>) -> DynCObject {
        DynCObject {
            ptr,
            len,
            lib,
        }
    }

    pub fn get(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr as *const u8, self.len as usize)
        }
    }

    pub fn set(&self, val: &[u8]) -> Result<(), Error> {
        let datalen = val.len();
        let buflen = self.len as usize;

        if datalen > buflen {
            Err(Error::BufferOverflow { buflen, datalen })
        } else {
            unsafe {
                let slice = slice::from_raw_parts_mut(
                    self.ptr as *mut u8,
                    self.len as usize
                );
                slice.copy_from_slice(val);
            }

            Ok(())
        }
    }
}

impl PartialEq for DynCObject {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BufferOverflow {
        buflen: usize,
        datalen: usize,
    }
}

impl std::error::Error for Error { }

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            Error::BufferOverflow { buflen, datalen } =>
                write!(
                    f,
                    "BufferOverflow: Cannot put data of size {} in buffer of size {}",
                    datalen,
                    buflen,
                ),
        }
    }
}
