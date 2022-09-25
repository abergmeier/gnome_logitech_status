
use rusb;

const VENDOR: u16 = 0x046d;
const PRODUCT: u16 = 0xc900;
const SWITCH_ON: [u8; 20] = [
    0x11, 0xff, 0x04, 0x1c, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const SWITCH_OFF: [u8; 20] = [
    0x11, 0xff, 0x04, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

pub struct LitraDevice {
    dev: rusb::Device<rusb::GlobalContext>,
}

pub struct LitraDevices {
    unfiltered: rusb::DeviceList<rusb::GlobalContext>,
}

impl LitraDevice {
    pub fn turn_on(self) {
        let handle = self.dev.open().unwrap();
        //let mut dummy = [u8, 0];

        //handle.write_interrupt(endpoint, &SWITCH_ON, Duration::from_secs(1));
        //handle.read_interrupt(endpoint, &mut dummy, Duration::from_secs(1)).unwrap();
        //time.Sleep(30 * time.Millisecond)
    }
    
    pub fn turn_off(self) {
        let handle = self.dev.open().unwrap();
        //let mut dummy = [u8, 0];
    
        //handle.write_interrupt(endpoint, &SWITCH_OFF, Duration::from_secs(1));
        //handle.read_interrupt(endpoint, &mut dummy, Duration::from_secs(1)).unwrap();
        //time.Sleep(30 * time.Millisecond)
    }
}

impl LitraDevices {
    pub fn new() -> Self {
        let unfiltered = rusb::devices().unwrap();
        LitraDevices { unfiltered }
    }

    fn can_handle<'r>(dev: &'r rusb::Device<rusb::GlobalContext>) -> bool {
        let desc = dev.device_descriptor().unwrap();
        match (desc.vendor_id(), desc.product_id()) {
            (VENDOR, PRODUCT) => (),
            _ => return false,
        }
        return desc.class_code() != rusb::constants::LIBUSB_CLASS_HID; // Skip HID devices, they are handled directly by OS libraries
    }

    pub fn iter<I>(self) -> impl Iterator<Item = LitraDevice> {
        let devices = self.unfiltered.iter().collect::<Vec<_>>();
        devices.into_iter().filter_map(|dev| {
            (Self::can_handle(&dev))
            .then_some(dev)
            .map(|dev| LitraDevice { dev })
        })
    }
}
