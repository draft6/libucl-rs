#!/bin/sh

bindgen wrapper.h -o src/lib.rs --whitelist-function "ucl_.*" --whitelist-type "ucl_.*" --default-enum-style rust --bitfield-enum "(ucl_parser_flags.*|ucl_string_flags.*|ucl_object_flags.*)" --opaque-type ucl_parser --blacklist-type __sbuf --blacklist-type __sFILE --blacklist-type __ubuf --blacklist-type __mbstate_t --blacklist-type FILE --raw-line '#![allow(non_camel_case_types)]' --blacklist-function "ucl_object_emit_file_funcs"
