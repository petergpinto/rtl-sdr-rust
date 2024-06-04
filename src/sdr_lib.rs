//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c

use log::{debug, error, info, trace};
use rusb::{Device, DeviceHandle, GlobalContext, Language};
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
    manufacturer: Option<String>,
    product: Option<String>,
    serial_number: Option<String>,
    tuner: Option<Tuner>,
    async_status: SdrAsyncStatus,
    usb_device: Device<GlobalContext>,
    usb_device_handle: Option<DeviceHandle<GlobalContext>>,
    dongle: SdrDongle,
    supported_languages: Option<Vec<Language>>,
    language: Option<Language>,
}

impl SdrDevice {
    pub fn new(usb: Device<GlobalContext>, dongle_type: SdrDongle) -> SdrDevice {
        SdrDevice {
            manufacturer: None,
            product: None,
            serial_number: None,
            tuner: None,
            async_status: SdrAsyncStatus::Inactive,
            usb_device: usb,
            dongle: dongle_type,
            usb_device_handle: None,
            language: None,
            supported_languages: None,
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

    pub fn open(&mut self) -> Result<(), rusb::Error>{
        let result = self.usb_device.open();
        self.usb_device_handle = match result {
            Ok(v) => {
                info!("Opened successfully");
                Some(v)
            }
            Err(e) => {
                error!("Encountered error attempting to open USB device: {e}");
                None
            }
        };
        info!("{:#?}", self.usb_device_handle);

        //Get metadata from the device
        if let Some(device_handle) = &self.usb_device_handle {
            debug!(
                "{:#?}",
                device_handle.read_languages(std::time::Duration::from_secs(1))
            );
            let language = device_handle
                .read_languages(std::time::Duration::from_secs(1))
                .unwrap()[0];
            let device = &device_handle.device().device_descriptor().unwrap();

            let manufacturer = device_handle
                .read_manufacturer_string(language, device, std::time::Duration::from_secs(1))
                .unwrap_or("Unknown".to_string());
            debug!("Manufacturer: {manufacturer}");
            let product = device_handle
                .read_product_string(language, device, std::time::Duration::from_secs(1))
                .unwrap_or("Unknown".to_string());
            debug!("Product Name: {product}");
            let serial_number = device_handle
                .read_serial_number_string(language, device, std::time::Duration::from_secs(1))
                .unwrap_or("Unknown".to_string());
            debug!("Serial number: {product}");

            device_handle.detach_kernel_driver(0)?;
            device_handle.claim_interface(0)?;

            self.manufacturer = Some(manufacturer);
            self.product = Some(product);
            self.serial_number = Some(serial_number);
            
        }
        Ok(())
    }

    pub fn describe(&mut self) {
        if let Some(device_handle) = &self.usb_device_handle {
            for interface in device_handle.device().config_descriptor(0).unwrap().interfaces() {
                for descriptor in interface.descriptors() {
                    trace!("{:#?}", descriptor);
                    for endpoint in descriptor.endpoint_descriptors() {
                        trace!("{:#?}", endpoint);
                    }                    
                } 
            }

        }
    }

    
    pub fn read(&mut self) {
        //Read should be a public interface that translates the request to an individual dongle type

        //Based on tuner type

        //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L405
        //https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L859

        match &self.usb_device_handle {
            Some(device_handle) => {
                let w_index = (6 << 8) | 0x10;
                const EEPROM_ADDR: u16 = 0xa0;
                let mut buffer: [u8; 1] = [1; 1];
                match device_handle.write_control(rusb::request_type(rusb::Direction::Out, rusb::RequestType::Vendor, rusb::Recipient::Device), 0, EEPROM_ADDR, w_index, &buffer, std::time::Duration::from_secs(1))
                {
                    Ok(v) => info!("Wrote buffer {:#?} to x{EEPROM_ADDR:04x}", buffer),
                    Err(e) => error!("Error reading from Device handle: {e}"),
                }
                let mut buffer: [u8; 16] = [1; 16]; //16 is largest size that works here 
                let w_index = (6 << 8);
                match device_handle.read_control(rusb::request_type(rusb::Direction::In, rusb::RequestType::Vendor, rusb::Recipient::Device), 0, EEPROM_ADDR, w_index, &mut buffer, std::time::Duration::from_secs(1))
                {
                    Ok(v) => info!("Read buffer {:#?} from x{EEPROM_ADDR:04x}", buffer),
                    Err(e) => error!("Error reading from Device handle: {e}"),
                }
                let mut buffer: [u8; 16] = [1; 16]; //16 is largest size that works here 
                match device_handle.read_control(rusb::request_type(rusb::Direction::In, rusb::RequestType::Vendor, rusb::Recipient::Device), 0, EEPROM_ADDR, w_index, &mut buffer, std::time::Duration::from_secs(1))
                {
                    Ok(v) => info!("Read buffer {:#?} from x{EEPROM_ADDR:04x}", buffer),
                    Err(e) => error!("Error reading from Device handle: {e}"),
                }
            }
            None => (),
        }
    }

    pub fn close(&mut self) -> Result<(), rusb::Error>{
        if let Some(device_handle) = &self.usb_device_handle {
            device_handle.release_interface(0)?;
            device_handle.attach_kernel_driver(0)?;
        }
        self.usb_device_handle = None;
        Ok(())
    }
}
