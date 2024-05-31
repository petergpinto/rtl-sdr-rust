//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/rtl_test.c

#[allow(clippy::needless_return, dead_code)]
mod sdr_lib;

use std::{collections::HashMap};

use log::{debug, error, info, trace, warn};
use rusb::{self, Device, GlobalContext};

use crate::sdr_lib::SdrDevice;
use sdr_lib::known_devices::KNOWN_DEVICES;
mod tuner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    //Search devices
    info!("Attempting to retrive USB devices from host...");
    let usb_devices = rusb::devices()?;

    let mut identified_sdr_devices: Vec<sdr_lib::SdrDevice> = vec![];
    for usb in usb_devices.iter() {
        let descriptor = usb.device_descriptor().map_err(|e| e.to_string())?;
        let product_id = descriptor.product_id();
        let vendor_id = descriptor.vendor_id();
        trace!("Checking Device - {product_id:04x}:{vendor_id:04x}");
        //PartialEq is implmented on SdrDongle to ignore the name field
        let possible_sdr = sdr_lib::SdrDongle {
            product_id,
            vendor_id,
            name: "UNKNOWN",
        };
        if let Some(index) = KNOWN_DEVICES.iter().position(|v| v == &possible_sdr) {
            let device = KNOWN_DEVICES[index];
            info!("Identified SDR Device {:#?}", device);
            identified_sdr_devices.push(SdrDevice::new(usb, device));
        }
    }
    if identified_sdr_devices.is_empty() {
        warn!("No SDR devices have been identified");
    }
    //identified_sdr_devices should now contain a list of `SdrDevice` objects that have valid USB device handles.

    for mut sdr in identified_sdr_devices {
        sdr.open()?;
        sdr.describe();
        sdr.read();
        sdr.close()?;
    }

    //Attempt to open each identified device
    //Report on device status
    Ok(())
}
