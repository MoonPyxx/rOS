#![no_std] // rust std library is not used  
#![no_main] // no entry level points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Meow World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle name
pub extern "C" fn _start() -> ! { // entry point, named _start
    let vga_buffer = 0xb8000 as *mut u8; // vga buffer address

    for (i, &byte) in HELLO.iter().enumerate() { // loop through each byte of HELLO
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write byte to vga buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f; // set color
        }
    }
    
    loop {}
}