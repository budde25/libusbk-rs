use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:include=libusbK/src/lib");

    println!("cargo:rustc-link-search=libusbK");
    println!("cargo:rustc-link-search=libusbK/libusbK/src/lib");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // generate_bindings();
    make_source();
}

fn generate_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .allowlist_function("LibK_.*")
        .allowlist_function("LstK_.*")
        .allowlist_function("OvlK_.*")
        .allowlist_function("UsbK_.*")
        .allowlist_function("StmK_.*")
        .allowlist_function("IsoK_.*")
        .allowlist_function("IsochK_.*")
        .allowlist_function("LUsb0_.*")
        .allowlist_function("HotK_.*")
        // types
        .allowlist_type("KHOT_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false) // comments are messed up
        .layout_tests(false)
        .derive_default(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn make_source() {
    let libusbk_source = PathBuf::from("libusbK");

    println!("cargo:vendored=1");
    println!("cargo:static=1");

    let include_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("include");
    fs::create_dir_all(&include_dir).unwrap();

    fs::copy(
        libusbk_source.join("libusbK/includes/libusbK.h"),
        include_dir.join("libusbK.h"),
    )
    .unwrap();

    fs::copy(
        libusbk_source.join("libusbK/includes/lusbk_shared.h"),
        include_dir.join("lusbk_shared.h"),
    )
    .unwrap();

    fs::copy(
        libusbk_source.join("libusbK/src/dll/lusbk_version.h"),
        include_dir.join("lusbk_version.h"),
    )
    .unwrap();

    println!("cargo:include={}", include_dir.to_str().unwrap());

    let src = libusbk_source.join("libusbK/src");

    let mut base_config = cc::Build::new();

    base_config.define("OS_WINDOWS", Some("1"));
    base_config.define("DEFAULT_VISIBILITY", Some(""));
    base_config.define("PLATFORM_WINDOWS", Some("1"));

    base_config.include(&include_dir);
    base_config.include(libusbk_source.join("libusbk"));
    base_config.include(&src);

    //base_config.file(libusbk_source.join("libusbK/includes/lusbk_dynapi.c"));

    link("setupapi", false);
    link("user32", false);
    link("setupapi", false);

    base_config.file(src.join("dll/lusbk_dllmain.c"));
    base_config.file(src.join("lusbk_usb.c"));
    base_config.file(src.join("lusbk_handles.c"));
    base_config.file(src.join("lusbk_bknd_libusbk.c"));
    base_config.file(src.join("lusbk_stack_collection.c"));
    base_config.file(src.join("lusbk_usb_iso.c"));
    base_config.file(src.join("lusbk_usb_isoch.c"));
    base_config.file(src.join("lusbk_ioctl.c"));
    base_config.file(src.join("lusbk_bknd_winusb.c"));
    base_config.file(src.join("lusbk_bknd_unsupported.c"));
    base_config.file(src.join("lusbk_bknd_libusb0.c"));
    base_config.file(src.join("lusbk_debug_view_output.c"));
    base_config.file(src.join("lusbk_device_list.c"));
    base_config.file(src.join("lusbk_wrapper_winusb.c"));
    base_config.file(src.join("lusbk_queued_stream.c"));
    base_config.file(src.join("lusbk_hot_plug.c"));

    base_config.compile("usb-vendored");
}

fn link(name: &str, bundled: bool) {
    use std::env::var;
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target.get(3) == Some(&"gnu") {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    }
}
