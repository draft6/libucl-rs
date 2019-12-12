use libucl_bind::*;

pub use self::types::Type;
pub use self::builder::Builder;
pub use self::emitter::Emitter;
use utils;

use std::convert::From;
use std::ffi::{
    CString
};
use libc::{
    c_void,
    c_uchar,
    c_double,
};

use std::fmt;

pub mod types;
pub mod builder;
pub mod emitter;

#[cfg(test)]
mod test;

// Helper functions
extern fn append_char(c: c_uchar, _num_chars: usize, ptr: *mut c_void) -> libc::c_int {
    assert!(!ptr.is_null());

    unsafe {
        let tmp = ptr as *mut String;
        (*tmp).push(c as char);
    }

    0
}

extern fn append_len(c: *const c_uchar, len: usize, ptr: *mut c_void) -> libc::c_int {
    assert!(!c.is_null());
    assert!(!ptr.is_null());

    unsafe {
        let out = ptr as *mut String;
        let slice = std::slice::from_raw_parts(c, len);
        (*out).push_str(std::str::from_utf8(slice).unwrap());
    }

    0
}

extern fn append_int(i: i64, ptr: *mut c_void) -> libc::c_int {
    assert!(!ptr.is_null());

    unsafe {
        let tmp = ptr as *mut String;
        let tmp_str = i.to_string();
        (*tmp).push_str(tmp_str.as_str());
    }

    0
}

extern fn append_double(d: c_double, ptr: *mut c_void) -> libc::c_int {
    assert!(!ptr.is_null());

    unsafe {
        let tmp = ptr as *mut String;
        let tmp_str = d.to_string();
        (*tmp).push_str(tmp_str.as_str());
    }

    0
}

/// File element object.
///
/// This structure is immutable typed reference to object inside parsed tree. It can be one of
/// `Type` elements and can be cast only to given type.
pub struct Object {
    obj: *mut ucl_object_t,
    it: ucl_object_iter_t,
    typ: Type
}

impl Object {
    /// Create new `Object` from const raw pointer. Internal use only.
    fn from_cptr(obj: *const ucl_object_t) -> Option<Self> {
        if !obj.is_null() {
            Some(Object {
                obj: unsafe { ucl_object_ref (obj) },
                it: std::ptr::null_mut(),
                typ: Type::from(unsafe { ucl_object_type(obj) })
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    /// Create new `Object` from mut raw pointer and take ownership. Internal use only.
    fn from_mut_cptr(obj: *mut ucl_object_t) -> Option<Self> {
        if !obj.is_null() {
            Some(Object {
                obj: obj,
                it: std::ptr::null_mut(),
                typ: Type::from(unsafe { ucl_object_type(obj) })
            })
        } else {
            None
        }
    }

    fn default_emit_funcs() -> ucl_emitter_functions {
        ucl_emitter_functions {
            ucl_emitter_append_character: Some(append_char),
            ucl_emitter_append_len: Some(append_len),
            ucl_emitter_append_int: Some(append_int),
            ucl_emitter_append_double: Some(append_double),
            ucl_emitter_free_func: None,
            ud: std::ptr::null_mut(),
        }
    }

    // pub fn priority(&self) -> usize {
    //     unsafe { ucl_object_get_priority(self.obj) as usize }
    // }

    pub fn dump_into(&self) {}

    pub fn dump(&self) -> String {
        let out: Box<String> = Box::new(String::new());
        let mut emit = Self::default_emit_funcs();

        unsafe {
            emit.ud = std::mem::transmute::<Box<String>, *mut c_void>(out);
            ucl_object_emit_full(self.obj, ucl_emitter_t::UCL_EMIT_JSON, &mut emit, std::ptr::null());
            let out_final: Box<String> = std::mem::transmute(emit.ud);
            return *out_final
        }
    }

    pub fn size(&self) -> usize {
        if self.typ == Type::Array {
            return unsafe { ucl_array_size(self.obj) }
        }

        0
    }

    pub fn at(&self, i: usize) -> Option<Object> {
        if self.typ == Type::Array {
            unsafe {
                let out = ucl_array_find_index(self.obj, i);

                return Object::from_cptr(out)
            }
        }

        None
    }

    pub fn iter_reset(&mut self) {
        if !self.it.is_null() {
            self.it = unsafe { ucl_object_iterate_reset(self.it, self.obj) }
        }
    }

    /// Return key assigned to object.
    pub fn key(&self) -> Option<String> {
        utils::to_str(unsafe { ucl_object_key(self.obj) })
    }

    /// Return type of object.
    pub fn get_type(&self) -> Type {
        self.typ
    }

    /// Return `i64` value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_int(), Some(10));
    ///
    /// let obj = libucl::object::Builder::from("test_string").build();
    /// assert_eq!(obj.as_int(), None);
    /// ```
    pub fn as_int(&self) -> Option<i64> {

        if self.get_type() != Type::Int { return None }

        unsafe {
            let out: *mut i64 = &mut 0i64;
            let res = ucl_object_toint_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    /// Return `f64` value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::object::Builder::from(10f64).build();
    /// assert_eq!(obj.as_float(), Some(10.0));
    ///
    /// let obj = libucl::object::Builder::from("test_string").build();
    /// assert_eq!(obj.as_float(), None);
    /// ```
    pub fn as_float(&self) -> Option<f64> {

        if self.get_type() != Type::Float { return None }

        unsafe {
            let out: *mut f64 = &mut 0f64;
            let res = ucl_object_todouble_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    /// Return boolean value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::object::Builder::from(true).build();
    /// assert_eq!(obj.as_bool(), Some(true));
    ///
    /// let obj = libucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {

        if self.get_type() != Type::Boolean { return None }

        unsafe {
            let out: *mut bool = &mut true;
            let res = ucl_object_toboolean_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    /// Return string value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::object::Builder::from("test_string").build();
    /// assert_eq!(obj.as_string(), Some("test_string".to_string()));
    ///
    /// let obj = libucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<String> {

        if self.get_type() != Type::String { return None }
        unsafe {
            let out = ucl_object_tostring(self.obj);

            utils::to_str(out)
        }
    }

    /// Fetch object under key
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::Parser::new().parse("a = b;").unwrap();
    /// assert_eq!(obj.fetch("a").unwrap().as_string(), Some("b".to_string()));
    /// ```
    pub fn fetch<T: AsRef<str>>(&self, key: T) -> Option<Object> {
        //use libucl_sys::ucl_object_lookup;

        if self.get_type() != Type::Object { return None }

        let k = CString::new(key.as_ref()).unwrap();
        unsafe {
            let out = ucl_object_lookup(self.obj, k.as_ptr());

            Object::from_cptr(out)
        }
    }

    /// Fetch object at the end of path delimeted by `.` (dot)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = libucl::Parser::new().parse("a = { b = c; }").unwrap();
    /// assert_eq!(obj.fetch_path("a.b").unwrap().as_string(), Some("c".to_string()));
    /// ```
    pub fn fetch_path<T: AsRef<str>>(&self, path: T) -> Option<Object> {

        if self.get_type() != Type::Object { return None }

        let p = CString::new(path.as_ref()).unwrap();
        unsafe {
            let out = ucl_object_lookup_path(self.obj, p.as_ptr());

            Object::from_cptr(out)
        }
    }
}

impl Iterator for Object {
    type Item = super::Object;

    fn next(&mut self) -> Option<Self::Item> {

        if self.it.is_null() {
           self.it = unsafe { ucl_object_iterate_new(self.obj) }
        }

        if self.typ != Type::Array {
            return None
        }

        let cur = unsafe { ucl_object_iterate_safe (self.it, true) };
        if cur.is_null() {
            return None
        }

        super::Object::from_cptr(cur)
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            if !self.it.is_null() { ucl_object_iterate_free(self.it); }
            if !self.obj.is_null() { ucl_object_unref(self.obj); }
        }
    }
}

impl AsRef<Object> for Object {
    fn as_ref(&self) -> &Self { self }
}

impl fmt::Debug for Object {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = Emitter::JSON.emit(&self);

        if string.is_some() {
            fmt.write_str(&string.unwrap())
        } else {
            Err(fmt::Error)
        }
    }
}
