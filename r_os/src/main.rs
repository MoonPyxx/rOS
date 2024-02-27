#![no_std] // rust std library is not used  
#![no_main] // no entry level points

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle name
pub extern "C" fn _start() -> ! { // entry point, named _start
    loop {}
}