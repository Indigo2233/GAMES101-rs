use std::process::Command;

fn main() {
    println!(r"cargo:rustc-link-search=native=Clib");
    println!("cargo:rerun-if-changed=Clib/OBJ_Loader.h");
    println!("cargo:rerun-if-changed=Clib/OBJ_Loader_C.cpp");
    Command::new("g++").args(["-shared", "-o", "Clib/libobjloader.so", "Clib/OBJ_Loader_C.cpp", "-lc", "-fPIC"]);
}