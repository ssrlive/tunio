fn main() {
    println!("cargo:rerun-if-changed=wintun/wintun_functions.h");

    if std::env::consts::OS == "windows" {
        let bindings = bindgen::Builder::default()
            .header("wintun/wintun_functions.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .allowlist_function("Wintun.*")
            .allowlist_type("WINTUN_.*")
            .allowlist_var("WINTUN_.*")
            .blocklist_type("_GUID")
            .blocklist_type("BOOL")
            .blocklist_type("BYTE")
            .blocklist_type("DWORD")
            .blocklist_type("DWORD64")
            .blocklist_type("GUID")
            .blocklist_type("HANDLE")
            .blocklist_type("LPCWSTR")
            .blocklist_type("NET_LUID")
            .blocklist_type("WCHAR")
            .blocklist_type("wchar_t")
            .dynamic_library_name("wintun")
            .dynamic_link_require_all(true)
            .opaque_type("NET_LUID")
            .clang_arg("--target=i686-pc-windows-msvc")
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        // let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let out_path = std::path::PathBuf::from("src").join("lib.rs");
        bindings.write_to_file(&out_path).expect("Couldn't write bindings!");

        let prelude = r#"#![allow(non_snake_case, non_camel_case_types)]
#![cfg(target_os = "windows")]

use windows_sys::core::GUID;
use windows_sys::core::PCWSTR as LPCWSTR;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::NetworkManagement::Ndis::NET_LUID_LH as NET_LUID;
pub type DWORD = core::ffi::c_ulong;
pub type BYTE = core::ffi::c_uchar;
pub type DWORD64 = core::ffi::c_ulonglong;

"#;

        let bindings = std::fs::read_to_string(&out_path).unwrap();
        let content0 = prelude.to_string() + &bindings;

        fn normalize_newlines(content: &str) -> String {
            // replace all \n with \r\n, but first replace all \r\n with a temp placeholder
            let temp_placeholder = "__TEMP__PLACEHOLDER__";
            let content_with_placeholder = content.replace("\r\n", temp_placeholder);
            let content_with_crlf = content_with_placeholder.replace("\n", "\r\n");
            content_with_crlf.replace(temp_placeholder, "\r\n")
        }

        let final_content = normalize_newlines(&content0);
        std::fs::write(&out_path, final_content).unwrap();
    }
}
