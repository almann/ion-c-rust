use cmake;

fn main() {
    let ionc = cmake::Config::new("ionc").build();

    println!("cargo:rustc-link-search=native={}/lib", ionc.display());
    println!("cargo:rustc-link-lib=static=decNumber_static");
    println!("cargo:rustc-link-lib=static=ionc_static");
}