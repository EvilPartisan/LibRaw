//use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=binding.rs");

    let bindings = bindgen::Builder::default()
        .header("libraw/libraw.h")
        .allowlist_function(r#"(\w*libraw\w*)"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //   let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        //  .write_to_file(out_path.join("bindings.rs"))
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
