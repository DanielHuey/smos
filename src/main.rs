#![no_std]
#![no_main]

use core::panic::PanicInfo;
//our own panic handler: ref(no_std)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

//our own entry point: ref(no_main)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8; //0xb8000 is the buffer called VGA Text Buffer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
