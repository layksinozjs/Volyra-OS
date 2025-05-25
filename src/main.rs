#![no_std]
#![no_main]

mod buffer;

use core::panic::PanicInfo;
use crate::buffer::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
println!("Welcome to macro of VolyraOS!");
    loop {}
}




#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}