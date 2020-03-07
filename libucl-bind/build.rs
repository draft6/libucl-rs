extern crate cmake;

use cmake::Config;


#[cfg(not(target_os = "freebsd"))]
fn main() {
    if std::env::var("LIBUCL_BIND_USE_SYSTEM").is_ok() {
        use_system_lib_ucl()
    } else {
        use_bundled_lib_ucl()
    }
}

#[cfg(target_os = "freebsd")]
fn main() {
    if std::env::var("LIBUCL_BIND_USE_BUNDLED").is_ok() {
        use_bundled_lib_ucl()
    } else {
        use_system_lib_ucl()
    }
}

fn use_bundled_lib_ucl() {
    let dst = Config::new("libucl")
        .no_build_target(true)
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=ucl");
}

fn use_system_lib_ucl() {
    println!("cargo:rustc-link-lib=ucl");
}