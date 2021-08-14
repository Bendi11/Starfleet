fn main() {
    if cfg!(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows"
    )) {
        println!("cargo:rustc-cfg=use_linkme")
    } else {
        println!("cargo:rustc-cfg=use_inventory")
    }
}
