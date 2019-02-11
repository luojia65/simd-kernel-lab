#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)] 
#![cfg_attr(test, allow(unused_imports))]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
