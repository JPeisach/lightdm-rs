fn main() {
    // Tell Cargo where to find the library file (e.g., /path/to/my/lib)
//    println!("cargo::rustc-link-search=native=/usr/lib");

    // Tell Cargo to link the library (e.g., looks for libfoo.a or libfoo.so)
    // Do not include the "lib" prefix or file extensions (.a, .so, .lib, .dll)
    println!("cargo::rustc-link-lib=lightdm-gobject-1");
}
