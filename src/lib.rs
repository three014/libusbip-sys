#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use libudev_sys::udev_device;

pub const SYSFS_PATH_MAX: usize = 256;
pub const SYSFS_BUS_ID_SIZE: usize = 32;
pub const USBIDS_FILE: &str = "/usr/share/hwdata/usb.ids";
pub const USBIP_VHCI_DRV_NAME: &str = "vhci_hcd";
pub const VHCI_STATE_PATH: &str = "/var/run/vhci_hcd";

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum hub_speed {
    HUB_SPEED_HIGH = 0,
    HUB_SPEED_SUPER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum usbip_device_status {
    /* sdev is available. */
	SDEV_ST_AVAILABLE = 0x01,
	/* sdev is now used. */
	SDEV_ST_USED,
	/* sdev is unusable because of a fatal error. */
	SDEV_ST_ERROR,

	/* vdev does not connect a remote device. */
	VDEV_ST_NULL,
	/* vdev is used, but the USB address is not assigned yet */
	VDEV_ST_NOTASSIGNED,
	VDEV_ST_USED,
	VDEV_ST_ERROR
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct usbip_imported_device {
    pub hub: hub_speed,
    pub port: u8,
    pub status: usbip_device_status,
    pub devid: u32,
    pub busnum: u8,
    pub devnum: u8,
    pub udev: usbip_usb_device
}

