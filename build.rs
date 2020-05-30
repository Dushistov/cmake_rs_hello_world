use std::path::Path;

fn main() {
    let dst = cmake::Config::new(Path::new("aws-c-common"))
        .build_target("all")
        .build()
        .join("build");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=aws-c-common");
}
