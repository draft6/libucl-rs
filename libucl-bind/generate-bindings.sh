#!/bin/sh

bindgen wrapper.h -o src/lib.rs  \
    --bitfield-enum "(ucl_parser_flags.*|ucl_string_flags.*|ucl_object_flags.*)" \
    --blacklist-function "ucl_object_emit_file_funcs" \
    --blacklist-type FILE \
    --blacklist-type __mbstate_t \
    --blacklist-type __sFILE \
    --blacklist-type __sbuf \
    --blacklist-type __ubuf \
    --default-enum-style rust \
    --opaque-type ucl_parser \
    --raw-line '#![allow(non_camel_case_types)]' \
    --size_t-is-usize \
    --whitelist-function "ucl_.*" \
    --whitelist-type "ucl_.*"
