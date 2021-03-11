extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let ros_lib_x86_64_linux_gnu = if let Ok(path) = env::var("ROS_DISTRO") {
    	format!("/opt/ros/{}/lib/x86_64-linux-gnu", path)
    } else { "/opt/ros/foxy/lib/x86_64-linux-gnu".into() };
    println!("cargo:rustc-link-search={}", ros_lib_x86_64_linux_gnu);
    println!("cargo:rustc-link-lib=ddsc");
    println!("cargo:rustc-link-lib=cdds-util");
    let incl_dir = if let Ok(path) = env::var("CYCLONE_INCLUDE") {
    	format!("-I{}", path)
    } else { "-I/usr/local/include".into() };
    let lib_dir = if let Ok(path) = env::var("CYCLONE_LIB") {
    	format!("-L{}", path)
    } else { "-L/usr/local/lib".into() };
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/9/include")
	.clang_arg(incl_dir)
	.clang_arg(lib_dir)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
