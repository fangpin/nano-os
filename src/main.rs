#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &c) in HELLO.iter().enumerate() {
        unsafe {
            // Each character takes 2 bytes: one for the character and one for the attribute
            *vga_buffer.offset(i as isize * 2) = c; 
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0b; 
        }
    }

    loop {} // Infinite loop to keep the program running
}
