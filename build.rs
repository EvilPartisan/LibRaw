//use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=binding.rs");
    println!("cargo::rustc-link-search=./lib");
    println!("cargo::rustc-link-lib=libraw_static");

    let bindings = bindgen::Builder::default()
        .header("libraw/libraw.h")
        .allowlist_item(r#"(\w*libraw\w*)"#)
        .allowlist_item(r#"(\w*LIBRAW\w*)"#)
        .allowlist_item(r#"(\w*LibRaw\w*)"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //   let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        //  .write_to_file(out_path.join("bindings.rs"))
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
