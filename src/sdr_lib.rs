//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c

use log::{error, info};
use rusb::{Device, DeviceHandle, GlobalContext};
pub mod constants;
pub mod known_devices;

struct Tuner {
    //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L52
    frequency: u32,
    bandwidth: i32,
    gain: i32,
    if_gain: i32,
    gain_mode: i32,
}

enum SdrAsyncStatus {
    //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L63
    Inactive = 0,
    Cancelling,
    Running,
}

//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L300
#[derive(Debug, Clone, Copy)]
pub struct SdrDongle {
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: &'static str,
}

impl PartialEq for SdrDongle {
    fn eq(&self, other: &Self) -> bool {
        if self.product_id == other.product_id && self.vendor_id == other.vendor_id {
            return true;
        }
        return false;
    }
}

//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L309

pub struct SdrDevice {
    //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L89
    manufacturer: &'static str,
    product: &'static str,
    tuner: Option<Tuner>,
    async_status: SdrAsyncStatus,
    usb_device: Device<GlobalContext>,
    usb_device_handle: Option<DeviceHandle<GlobalContext>>,
    dongle: SdrDongle,
}

impl SdrDevice {
    pub fn new(usb: Device<GlobalContext>, dongle_type: SdrDongle) -> SdrDevice {
        SdrDevice {
            manufacturer: dongle_type.name.split("").collect::<Vec<&str>>()[0],
            product: dongle_type.name,
            tuner: None,
            async_status: SdrAsyncStatus::Inactive,
            usb_device: usb,
            dongle: dongle_type,
            usb_device_handle: None,
        }
    }

    pub fn set_gpio_bit(&mut self, gpio: u8, value: i32) {
        //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L554
        unimplemented!()
    }

    pub fn set_if_frequency(&mut self, frequency: u32) {
        //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L686
        unimplemented!()
    }

    pub fn open(&mut self) {
        self.usb_device_handle = self.usb_device.open().ok();
        info!("{:#?}", self.usb_device_handle);
    }

    pub fn read(&mut self) {
        match &self.usb_device_handle {
            Some(device_handle) => {
                let mut buffer: [u8; 1000] = [0; 1000]; 
                match device_handle.read_bulk(1, &mut buffer, std::time::Duration::from_secs(1)) {
                    Ok(v) => info!("Read to buffer {:#?}", buffer),
                    Err(e) => error!("{e}"),
                }
            }
            None => ()
        }
    }

    pub fn close(&mut self) {
        self.usb_device_handle = None
    }
}
