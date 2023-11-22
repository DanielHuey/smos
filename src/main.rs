#![no_std]
#![no_main]

use core::panic::PanicInfo;
//our own panic handler: ref(no_std)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//our own entry point: ref(no_main)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
