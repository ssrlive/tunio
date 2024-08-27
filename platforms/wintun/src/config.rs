use derive_builder::Builder;
use tunio_core::traits::PlatformIfConfigT;
use windows_sys::Win32::System::Com::CoCreateGuid;

/// It is generally better to use [`PlatformIfConfigBuilder`] to create a new PlatformIfConfig instance.
#[derive(Builder, Clone)]
pub struct PlatformIfConfig {
    /// Wintun ring capacity. Must be power of 2 between 128KiB and 64MiB
    #[builder(default = "2 * 1024 * 1024")]
    pub capacity: u32,
    #[builder(default = "String::new()")]
    pub description: String,
    /// GUID of this network interface. It is recommended to set it manually,
    /// or new device will be created on each invocation, and it will quickly
    /// pollute Windows registry.
    #[builder(default = "guid_to_u128(&new_guid())")]
    pub guid: u128,
}

pub fn new_guid() -> windows_sys::core::GUID {
    unsafe {
        let mut guid = std::mem::zeroed();
        let _ = CoCreateGuid(&mut guid);
        guid
    }
}

pub fn guid_to_u128(guid: &windows_sys::core::GUID) -> u128 {
    (guid.data1 as u128) << 96
        | (guid.data2 as u128) << 80
        | (guid.data3 as u128) << 64
        | u64::from_le_bytes(guid.data4) as u128
}

impl Default for PlatformIfConfig {
    fn default() -> Self {
        PlatformIfConfigBuilder::default().build().unwrap()
    }
}

impl PlatformIfConfigT for PlatformIfConfig {
    type Builder = PlatformIfConfigBuilder;
}
