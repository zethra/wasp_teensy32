#![no_std]
#![feature(lang_items)]

#[macro_use]
extern crate teensy3;
extern crate wasp;

use teensy3::bindings;
use teensy3::serial::Serial;

use wasp::hardware::HardwareGpio;
use wasp::hardware::HardwareTime;
use wasp::hardware::HardwareUart;
use wasp::hardware::PinMode;
use wasp::hardware::PinState;

pub struct HardwareTeensy3 {
    serial: Serial,
}

impl HardwareTeensy3 {
    pub fn new() -> HardwareTeensy3 {
        HardwareTeensy3 {
            serial: Serial{}
        }
    }
}

impl HardwareGpio for HardwareTeensy3 {
    fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            bindings::pinMode(pin, match mode {
                PinMode::Input => teensy3::bindings::INPUT,
                PinMode::Output => teensy3::bindings::OUTPUT,
                PinMode::InputPullup => teensy3::bindings::INPUT_PULLUP,
                PinMode::InputPulldown => teensy3::bindings::INPUT_PULLDOWN,
                PinMode::OutputOpenDrain => teensy3::bindings::OUTPUT_OPENDRAIN,
            } as u8);
        }
    }

    fn digital_write(&self, pin: u8, val: PinState) {
        unsafe {
            bindings::digitalWrite(pin, val as u8);
        }
    }

    fn digital_read(&self, pin: u8) -> PinState {
        unsafe {
            if bindings::digitalRead(pin) == 0u8 {PinState::Low} else {PinState::High}
        }
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

impl HardwareUart for HardwareTeensy3 {
    fn readable(&self) -> bool {
        self.serial.readable()
    }

    fn try_read_byte(&self) -> Result<u8, &'static str> {
        self.serial.try_read_byte()
    }

    fn write_bytes(&self, bytes: &[u8]) -> Result<(), ()> {
        self.serial.write_bytes(bytes)
    }
}
