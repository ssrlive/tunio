#![allow(dead_code)]
use crate::wrappers::HandleWrapper;
use windows_sys::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE},
    System::Threading::{CreateEventA, SetEvent},
};

pub(crate) struct SafeEvent(HandleWrapper<HANDLE>);

impl SafeEvent {
    pub fn new(manual_reset: bool, initial_state: bool) -> Self {
        let m = manual_reset as BOOL;
        let i = initial_state as BOOL;
        let handle = unsafe { CreateEventA(std::ptr::null(), m, i, std::ptr::null()) };
        Self(HandleWrapper(handle))
    }

    pub fn set_event(&self) {
        unsafe {
            SetEvent(self.handle().0);
        }
    }

    pub fn handle(&self) -> HandleWrapper<HANDLE> {
        self.0
    }
}

impl Drop for SafeEvent {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.0 .0) };
    }
}
