#![no_std]

use stm32f1xx_hal::dma::*;
use stm32f1xx_hal::spi::*;

struct DeviceDriver<SPI> {
    spi: Option<SPI>,
    buf: Option<&'static mut [u8]>,
}

impl<SPI, REMAP, PINS, CH> DeviceDriver<SpiTxDma<SPI, REMAP, PINS, CH>>
where
    SpiTxDma<SPI, REMAP, PINS, CH>: WriteDma<&'static mut [u8], u8>,
{
    pub fn new(spi: SpiTxDma<SPI, REMAP, PINS, CH>, buf: &'static mut [u8]) -> Self {
        Self {
            spi: Some(spi),
            buf: Some(buf),
        }
    }

    pub fn send(&mut self, data: impl Iterator<Item = [u8; 3]>) {
        if let (Some(spi), Some(buf)) = (self.spi.take(), self.buf.take()) {
            let txdma = spi.write(buf);
            let (buf, spi) = txdma.wait(); // Doesn't compile
            self.buf = Some(buf);
            self.spi = Some(spi);
        }
    }
}
