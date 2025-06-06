#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[entry]
fn main() -> ! {
    loop {}
}
