use anyhow::Result;

use ddc::DdcHost;
use ddc_hi::{Ddc, Display};

pub fn get_display() -> Option<Display> {
    Display::enumerate().pop()
}

pub fn set_brightness(display: &mut Display, value: u16) -> Result<()> {
    let result = display.handle.set_vcp_feature(0x10, value);
    display.handle.sleep();
    result
}
