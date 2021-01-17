#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "crc8")]
pub mod crc8;

#[cfg(feature = "fmp_uart")]
pub mod fmp_uart;

#[cfg(feature = "fmp_uart_fmprinter")]
pub mod fmp_uart_fmprinter;

#[cfg(feature = "fmp_usb")]
pub mod fmp_usb;

#[cfg(test)]
mod test;
