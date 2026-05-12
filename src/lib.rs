#![no_std]

use core::panic::PanicInfo;

pub extern "system" fn driver_entry() -> u32 {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {  }
}
