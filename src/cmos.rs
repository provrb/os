use crate::time::DateTime;
use x86_64::instructions::port::Port;

const CMOS_ADDR: u16 = 0x70; // write only
const CMOS_DATA: u16 = 0x71; // read and write

pub static mut CMOS_INSTANCE: CMOS = CMOS::init();

#[derive(Debug)]
enum CMOSRTCRegister {
    Seconds = 0x00,    // 0-59
    Minutes = 0x02,    // 0-59
    Hours = 0x04,      // 24 hour clock, 0-23
    Weekday = 0x06,    // 1-7
    DayOfMonth = 0x07, // 1-31
    Month = 0x08,      // 1-12
    Year = 0x09,       // last 2 digits of the year (e.g 25 for 2025)

    StatusA = 0x0A,
    StatusB = 0x0B,
    StatusC = 0x0C,
    StatusD = 0x0D,
}

impl CMOSRTCRegister {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Convert binary-coded decimal to normal binary numbers
pub(crate) fn bcd_to_binary(byte: u8) -> u8 {
    (byte & 0x0F) + ((byte / 16) * 10)
}

pub(crate) struct CMOS {
    address_register: Port<u8>,
    data_register: Port<u8>,
}

impl CMOS {
    const fn init() -> Self {
        Self {
            address_register: Port::new(CMOS_ADDR),
            data_register: Port::new(CMOS_DATA),
        }
    }

    /// This function is unsafe because the caller
    /// needs to guarentee that the register they are
    /// writing to has write permissions and exists.
    /// If not, this can lead to undefined behavour.
    pub(crate) unsafe fn write_register(&mut self, data: u8) {
        self.address_register.write(data);
    }

    /// Read from the CMOS data register
    ///
    /// This function is unsafe because .read can cause
    /// side effects that are not memory safe
    pub(crate) unsafe fn read_cmos(&mut self) -> u8 {
        // write to cmos address register
        self.data_register.read()
    }

    /// Check the StatusB register
    /// Check bit 2, value 4, 1 = binary mode set
    pub(crate) fn is_bcd_mode(&mut self) -> bool {
        unsafe {
            self.write_register(CMOSRTCRegister::StatusB.as_u8());
            self.read_cmos() & 0x04 == 0
        }
    }

    /// Check the status b register
    /// Check bit 1, value = 2, 1 = hours are in 24 format
    pub(crate) fn is_24_hour_format(&mut self) -> bool {
        unsafe {
            self.write_register(CMOSRTCRegister::StatusB.as_u8());
            self.read_cmos() & 0x02 == 0
        }
    }

    pub(crate) unsafe fn time_now(&mut self) -> DateTime {
        let mut time = DateTime::new(0, 0, 0, 0, 0, 0);

        self.write_register(CMOSRTCRegister::Seconds.as_u8()); // seconds
        time.second = self.read_cmos();

        self.write_register(CMOSRTCRegister::Minutes.as_u8());
        time.minute = self.read_cmos();

        self.write_register(CMOSRTCRegister::Hours.as_u8());
        time.hour = self.read_cmos();

        self.write_register(CMOSRTCRegister::DayOfMonth.as_u8());
        time.day = self.read_cmos();

        self.write_register(CMOSRTCRegister::Month.as_u8());
        time.month = self.read_cmos();

        self.write_register(CMOSRTCRegister::Year.as_u8());
        time.year = self.read_cmos();

        time
    }
}
