fn main() {
    // Set up the bindings for the MKL library
    let bindings = bindgen::Builder::default()
        .header("/opt/intel/oneapi/mkl/2024.0/include/mkl_dfti.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .generate()
        .expect("Unable to generate mkl bindings");

    // Write the bindings directly to the src directory
    let out_path = std::path::PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("mkl_bindings.rs"))
        .expect("Couldn't write mkl bindings!");

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