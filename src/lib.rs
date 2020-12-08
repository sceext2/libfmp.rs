#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "fmp_uart")]
pub mod fmp_uart;

#[cfg(feature = "fmp_uart_fmprinter")]
pub mod fmp_uart_fmprinter;

#[cfg(test)]
mod test;
