use super::HandleWrapper;
use log::error;
use std::io;
use std::sync::Arc;
use tunio_core::Error;
use windows_sys::{core::GUID, Win32::NetworkManagement::Ndis::NET_LUID_LH};
use wintun_sys::WINTUN_ADAPTER_HANDLE;

pub struct Adapter {
    wintun: Arc<wintun_sys::wintun>,
    handle: HandleWrapper<WINTUN_ADAPTER_HANDLE>,
}

impl Adapter {
    pub fn new(guid: GUID, name: &str, description: &str, wintun: Arc<wintun_sys::wintun>) -> Result<Self, Error> {
        let name_u16 = string_to_utf16_null_terminated(name);
        let description_u16 = string_to_utf16_null_terminated(description);

        let adapter_handle =
            unsafe { wintun.WintunCreateAdapter(name_u16.as_ptr(), description_u16.as_ptr(), &guid as *const _ as _) };

        if adapter_handle.is_null() {
            let err = io::Error::last_os_error();
            error!("Failed to create adapter: {err}");
            return Err(Error::from(err));
        }

        Ok(Self {
            wintun,
            handle: HandleWrapper(adapter_handle),
        })
    }

    pub fn luid(&self) -> u64 {
        let mut luid_buf = unsafe { std::mem::zeroed::<NET_LUID_LH>() };
        unsafe {
            self.wintun
                .WintunGetAdapterLUID(self.handle.0, &mut luid_buf as *mut _ as _);
            luid_buf.Value
        }
    }

    pub fn handle(&self) -> HandleWrapper<WINTUN_ADAPTER_HANDLE> {
        self.handle
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        unsafe { self.wintun.WintunCloseAdapter(self.handle.0) };
    }
}

fn string_to_utf16_null_terminated(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
