use libucl_bind::*;
use libc::size_t;

use utils;
use error;

use object::{
    self,
    Object
};

use super::Result;

use std::path::Path;
use std::ffi::CString;

bitflags! {
    pub struct Flags: i32 {
        const DEFAULT            = 0x0;
        const LOWERCASE          = 0x1;
        const ZEROCOPY           = 0x2;
        const NO_TIME            = 0x4;
        const NO_IMPLICIT_ARRAYS = 0x8;
    }
}

pub struct Parser {
    parser: *mut ucl_parser,
}

impl Parser {
    /// Create new parser instance with default options
    pub fn new() -> Self {
        Self::with_flags(Flags::DEFAULT)
    }

    /// Create new parser with given option flags
    ///
    /// Flags:
    ///
    /// - `DEFAULT` - default configuration
    /// - `LOWERCASE` - convert all keys to lower case
    /// - `ZEROCOPY` - parse input in zero-copy mode if possible (you must ensure that input memory
    ///   is not freed if an object is in use)
    /// - `NO_TIME` - do not parse time and treat it's value as string
    /// - `NO_IMPLICIT_ARRAYS` - create explicit arrays instead of implicit ones
    ///
    /// # Examples
    ///
    /// ```rust
    /// let parser = libucl::Parser::with_flags(libucl::parser::Flags::LOWERCASE);
    /// let doc = parser.parse("A = b").unwrap();
    ///
    /// assert!(doc.fetch("a").is_some());
    /// ```
    pub fn with_flags(flags: Flags) -> Self {
        Parser {
            parser: unsafe { ucl_parser_new(flags.bits()) }
        }
    }

    /// Parse given string. Returns root object on success.
    ///
    /// It moves out `Parser`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert!(libucl::Parser::new().parse("a = b").is_ok());
    /// assert!(libucl::Parser::new().parse("a =").is_err());
    /// ```
    pub fn parse<T: AsRef<str>>(mut self, string: T) -> Result<Object> {
        let len = string.as_ref().len() as size_t;
        let s = CString::new(string.as_ref()).unwrap();
        let result = unsafe { ucl_parser_add_chunk(self.parser, s.as_ptr(), len) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error())
        }
    }

    /// Parse file at given `Path`.
    ///
    pub fn parse_file<T: AsRef<Path>>(mut self, path: T) -> Result<Object> {
        let filename = path.as_ref().to_str().unwrap();
        let s = CString::new(filename).unwrap();
        let result = unsafe { ucl_parser_add_file(self.parser, s.as_ptr()) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error())
        }
    }

    /// Register new variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// let p = libucl::Parser::new();
    /// p.register_var("someVar".to_string(), "test_string".to_string());
    /// let res = p.parse("testVar = $someVar").unwrap();
    ///
    /// assert_eq!(res.fetch("testVar").unwrap().as_string(), Some("test_string".to_string()));
    /// ```
    pub fn register_var(&self, name: String, value: String) {
        let n = CString::new(name).unwrap();
        let v = CString::new(value).unwrap();
        unsafe {
            ucl_parser_register_variable(self.parser, n.as_ptr(), v.as_ptr())
        }
    }

    fn get_object(&mut self) -> Option<Object> {
        object::Builder::from_ptr(unsafe { ucl_parser_get_object(self.parser) }).map(|o| o.build())
    }

    fn get_error(&mut self) -> error::Error {
        let err = unsafe { ucl_parser_get_error_code(self.parser) };
        let desc = unsafe { ucl_parser_get_error(self.parser) };

        error::Error::from_code(err, utils::to_str(desc).unwrap())
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe { ucl_parser_free(self.parser) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_parsing() {
        let p = Parser::new();
        let s = r#"test_string = "test_string""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn empty_string_parsing() {
        let p = Parser::new();
        let s = r#""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn key_fetching() {
        let p = Parser::new();
        let s = r#"test_var = 10"#;
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("test_var").unwrap().as_int(), Some(10));
    }

    #[test]
    fn flags() {
        let s = r#"test_Var = 10"#;
        let p = Parser::with_flags(Flags::DEFAULT);
        let res = p.parse(s).unwrap();

        assert!(res.fetch("test_var").is_none());

        let p = Parser::with_flags(Flags::LOWERCASE);
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("test_var").unwrap().as_int(), Some(10));
    }

    #[test]
    fn variables() {
        let s = r#"testVar = $ENV"#;
        let p = Parser::new();
        p.register_var("ENV".to_string(), "test".to_string());
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("testVar").unwrap().as_string(), Some("test".to_string()));
    }

    #[test]
    fn parse_array_and_iter() {
        let parser = Parser::new();
        let result = parser.parse(r#"name = "test_string";
            section {
                nice = true;
                server = ["http://localhost:6666", "test_string"];
                chunk = 1Gb;
            }"#).unwrap();
        let val = result.fetch_path("section.server");
        assert!(val.is_some());

        let mut obj = val.unwrap();
        assert_eq!(obj.get_type() == object::Type::Array, true);
        assert_eq!(obj.next().unwrap().as_string().unwrap(), "http://localhost:6666");
        assert_eq!(obj.next().unwrap().as_string().unwrap(), "test_string");
        assert_eq!(obj.next().is_none(), true);

        let val = result.fetch_path("section.server").unwrap();
        for o in val {
            assert_ne!(o.as_string(), None);
        }

    }

    #[test]
    fn object_dump() {
        let parser = Parser::new();
        let result = parser.parse(r#"name = "test_string";
            section {
                nice = true;
                server = ["http://localhost:6666", "test_string"];
                chunk = 1Gb;
            }"#).unwrap();
        let val = result.fetch_path("section.server");
        assert!(val.is_some());
        assert_eq!(result.dump().len(), 138);

    }

}
