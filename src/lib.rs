#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use libudev_sys::udev_device;

pub const SYSFS_PATH_MAX: usize = 256;
pub const SYSFS_BUS_ID_SIZE: usize = 32;
pub const USBIDS_FILE: &str = "/usr/share/hwdata/usb.ids";
pub const USBIP_VHCI_DRV_NAME: &str = "vhci_hcd";

#[repr(u32)]
pub enum hub_speed {
    HUB_SPEED_HIGH = 0,
    HUB_SPEED_SUPER,
}


fn foo() {
}
