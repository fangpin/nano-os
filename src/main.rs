#![no_std]
#![no_main]

use os::vga_buffer;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::write_something();

    loop {} // Infinite loop to keep the program running
}
