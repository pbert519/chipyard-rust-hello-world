use core::fmt::{Error, Write};

pub struct Uart{
    base_address: usize,
}

const TXDATA: usize = 0;
const RXDATA: usize = 1;
const TXCTRL: usize = 2;
const RXCTRL: usize = 3;
const IE: usize = 4;
const IP: usize = 5;
const DIV: usize = 6;



impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}


impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u32;
        unsafe {
            ptr.add(DIV).write_volatile(867); // div
            ptr.add(TXCTRL).write_volatile(0x00000001); // txctrl
            ptr.add(RXCTRL).write_volatile(0x00000001); // rxctrl
            ptr.add(IE).write_volatile(0); // ie
        }

    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u32; 
        unsafe {
            ptr.add(TXDATA).write_volatile(c as u32);
        }
    }
}