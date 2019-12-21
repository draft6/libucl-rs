extern crate cmake;

use cmake::Config;

fn main() {
    
    let dst = Config::new("libucl")
                     .no_build_target(true)
                     .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=ucl");
}
