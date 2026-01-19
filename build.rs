//use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=binding.rs");
    println!("cargo::rustc-link-search=./lib");
    println!("cargo::rustc-link-lib=libraw_static");

    let bindings = bindgen::Builder::default()
        .header("libraw/libraw.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //   let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        //  .write_to_file(out_path.join("bindings.rs"))
        .write_to_file("src/libraw-sys.rs")
        .expect("Couldn't write bindings!");
}
