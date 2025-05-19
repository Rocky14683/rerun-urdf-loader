
fn main() {
    let test = cmake::Config::new("cpp/test").build();
    let urdf_parser = cmake::Config::new("cpp/URDF_Parser").build();

    println!("cargo:rustc-link-search=native={}/lib", test.display());
    println!("cargo:rustc-link-search=native={}/lib", urdf_parser.display());
    println!("cargo:rustc-link-lib=static=mylib");
    println!("cargo:rustc-link-lib=static=urdfparser");
    println!("cargo:rustc-link-lib=c++");
}
