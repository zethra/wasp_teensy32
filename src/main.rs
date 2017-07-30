#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate wasp;
extern crate wasp_teensy32;

use wasp::hardware::HardwareGpio;
use wasp::hardware::HardwareTime;
use wasp::hardware::PinMode;
use wasp::stepper;
use wasp::utils::Point3;

use wasp_teensy32::teensy32::HardwareTeensy3;

const LINE_ENDING: u8 = 10;
const BUFFER_SIZE: usize = 256;
const LED_PIN: u8 = 13;

#[no_mangle]
pub unsafe extern "C" fn main() {

    let hardware = HardwareTeensy3::new();

    hardware.delay(5000000);
}
