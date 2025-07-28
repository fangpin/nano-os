#![no_std]
#![no_main]
mod vga_buffer;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !
{
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !
{
    write_something();
    loop {} // Infinite loop to keep the program running
}

fn write_something ()
{
    println!("print by println");
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello world");
    vga_buffer::WRITER.lock().new_line();
    write!(vga_buffer::WRITER.lock(), "The numbers are {} and {}", 42, 1).unwrap();
}
