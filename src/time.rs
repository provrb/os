use core::u8;
use crate::cmos::{bcd_to_binary, CMOS_INSTANCE};

#[derive(Debug)]
pub struct DateTime {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    pub fn new(year: u8, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn now() -> Self {
        // read time from CMOS
        let mut raw_time = unsafe { CMOS_INSTANCE.time_now() };
        
        // if cmos isnt in bcd mode than just return as is
        if !unsafe { CMOS_INSTANCE.is_bcd_mode() } {
            return raw_time;
        }

        // convert each file from bcd to binary
        raw_time.second = bcd_to_binary(raw_time.second);
        raw_time.minute = bcd_to_binary(raw_time.minute);
        raw_time.hour = bcd_to_binary(raw_time.hour);
        raw_time.day = bcd_to_binary(raw_time.day);
        raw_time.month = bcd_to_binary(raw_time.month);
        raw_time.year = bcd_to_binary(raw_time.year);
        raw_time
    }
}
