fn main() {
    // Link against the MKL library
    println!("cargo:rerun-if-env-changed=MKLROOT");
    println!("cargo:rustc-link-search=native=/opt/intel/oneapi/mkl/2024.0/lib/");
    println!("cargo:rustc-link-lib=static=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=dylib=mkl_core");
    println!("cargo:rustc-link-lib=dylib=mkl_sequential");
    println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = format!("{}/lib", crate_dir);

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search=native={}", lib_dir);
}