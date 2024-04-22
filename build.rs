use cmake::Config;

fn main() {
    let dst = Config::new("libtest")
        .generator("Ninja")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=test");
}
