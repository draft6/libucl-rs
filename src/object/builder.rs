use libucl_sys::*;

use super::Object;

use std::convert::From;
use std::ffi::CString;

/// Build element object.
///
/// This structure is immutable typed reference to object inside parsed tree. It can be one of
/// `Type` elements and can be cast only to given type.
pub struct Builder {
    obj: *mut ucl_object_t
}

impl Builder {
    /// Create new `Object` from raw pointer. Internal use only.
    pub fn from_ptr(obj: *mut ucl_object_t) -> Option<Self> {
        if !obj.is_null() {
            Some(Builder {
                obj: obj,
            })
        } else {
            None
        }
    }

    pub fn build(self) -> Object {
        Object::from_cptr(self.obj).unwrap()
    }
}

impl Into<Object> for Builder {
    fn into(self) -> Object {
        self.build()
    }
}

macro_rules! from_primitive {
    ($from: ty => $type: ident, $func: ident) => {
        impl From<$from> for Builder {
            fn from(val: $from) -> Self {
                //use libc;
                Builder::from_ptr(unsafe { $func(val as $type) }).unwrap()
            }
        }
    };

    ($from: ty => $primitive: ty, $func: ident) => {
        impl From<$from> for Builder {
            fn from(val: $from) -> Self {
                //use libc;
                Builder::from_ptr(unsafe { $func(val as $primitive) }).unwrap()
            }
        }
    };

    ($from: ty, $func: ident) => {
        impl From<$from> for Builder {
            fn from(val: $from) -> Self {
                Builder::from_ptr(unsafe { $func(val) }).unwrap()
            }
        }
    }
}

from_primitive!(i64 => i64, ucl_object_fromint);
from_primitive!(f64 => libc::c_double, ucl_object_fromdouble);
from_primitive!(bool, ucl_object_frombool);

impl From<String> for Builder {
    fn from(val: String) -> Self {
        //use libc;
        //use libucl_sys::ucl_object_fromlstring;

        let len = val.len();
        let s = CString::new(val).unwrap();
        let p = s.as_ptr();
        Builder::from_ptr(unsafe { ucl_object_fromlstring(p, len as libc::size_t) }).unwrap()
    }
}

impl<'a> From<&'a str> for Builder {
    fn from(val: &str) -> Self {
        From::from(val.to_string())
    }
}
