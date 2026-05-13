#![no_std]

use core::panic::PanicInfo;
use wdk_alloc::WdkAllocator;
use wdk_sys::{
    PDRIVER_OBJECT,
    NTSTATUS,
    PCUNICODE_STRING,
};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[cfg(not(test))]
extern crate wdk_panic;

// SAFETY: "DriverEntry" is the required symbol name for Windows driver entry points.
// No other function in this compilation unit exports this name, preventing symbol conflicts.
#[unsafe(export_name = "DriverEntry")] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
   driver: &mut DRIVER_OBJECT,
   registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
   0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {  }
}
