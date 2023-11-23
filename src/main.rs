#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
//our own panic handler: ref(no_std)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// static HELLO: &str = "Hello world!";
// static PROMPT: &str = "> ";

//our own entry point: ref(no_main)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer: *mut u8 = 0xb8000 as *mut u8; //0xb8000 is the buffer called VGA Text Buffer
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello again\n")
        .unwrap();
    println!("More formats {}", "HERE!!!");
    println!("Hello World again and again {} times", 3);
    // panic!("Finished!");
    loop {}
}
