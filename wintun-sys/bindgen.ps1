bindgen `
    --allowlist-function "Wintun.*" `
    --allowlist-type "WINTUN_.*" `
    --allowlist-var "WINTUN_.*" `
    --blocklist-type "_GUID" `
    --blocklist-type "BOOL" `
    --blocklist-type "BYTE" `
    --blocklist-type "DWORD" `
    --blocklist-type "DWORD64" `
    --blocklist-type "GUID" `
    --blocklist-type "HANDLE" `
    --blocklist-type "LPCWSTR" `
    --blocklist-type "NET_LUID" `
    --blocklist-type "WCHAR" `
    --blocklist-type "wchar_t" `
    --dynamic-loading wintun `
    --dynamic-link-require-all `
    --opaque-type "NET_LUID" `
    wintun/wintun_functions.h > src/lib.rs `
    -- --target=i686-pc-windows-msvc

# Insert prelude to generated file
$prelude = @'
#![allow(non_snake_case, non_camel_case_types)]
#![cfg(target_os = "windows")]

use windows_sys::core::GUID;
use windows_sys::core::PCWSTR as LPCWSTR;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::NetworkManagement::Ndis::NET_LUID_LH as NET_LUID;
pub type DWORD = core::ffi::c_ulong;
pub type BYTE = core::ffi::c_uchar;
pub type DWORD64 = core::ffi::c_ulonglong;

'@

# Write prelude to a tmp file
$tmpFile = "src/lib.rs.tmp"
$prelude | Out-File -FilePath $tmpFile -Encoding utf8

# Append generated file to tmp file
Get-Content src/lib.rs | Add-Content -Path $tmpFile

# Replace generated file with tmp file
Move-Item -Path $tmpFile -Destination src/lib.rs -Force
