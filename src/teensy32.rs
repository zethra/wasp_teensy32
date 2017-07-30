use teensy3;
use teensy3::bindings;

use wasp::hardware::HardwareGpio;
use wasp::hardware::HardwareTime;
use wasp::hardware::PinMode;

#[derive(Debug)]
pub struct HardwareTeensy3 {

}

impl HardwareTeensy3 {
    pub fn new() -> HardwareTeensy3 {
        HardwareTeensy3 {}
    }
}

impl HardwareGpio for HardwareTeensy3 {
    fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            bindings::pinMode(pin, match mode {
                PinMode::Input => bindings::INPUT,
                PinMode::Output => bindings::OUTPUT,
                PinMode::InputPullup => bindings::INPUT_PULLUP,
                PinMode::InputPulldown => bindings::INPUT_PULLDOWN,
                PinMode::OutputOpenDrain => bindings::OUTPUT_OPENDRAIN,
            } as u8);
        }
    }

    fn digital_write(&self, pin: u8, val: bool) {
        teensy3::util::digital_write(pin, val);
    }

    fn digital_read(&self, pin: u8) -> Option<bool> {
        Some(teensy3::util::digital_read(pin))
    }
}

impl HardwareTime for HardwareTeensy3 {
    fn delay(&self, micros: u32) {
        teensy3::util::delay(micros / 1000);
    }

    fn now(&self) -> u32 {
        unsafe {
            teensy3::bindings::micros()
        }
    }
}
