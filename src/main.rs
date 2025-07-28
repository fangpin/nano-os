#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::fmt::Write;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    write_something();

    #[cfg(test)]
    test_main();

    loop {} // Infinite loop to keep the program running
}

fn write_something() {
    println!("print by println");
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello world");
    vga_buffer::WRITER.lock().new_line();
    write!(
        vga_buffer::WRITER.lock(),
        "The numbers are {} and {}",
        42,
        1
    )
    .unwrap();
    vga_buffer::WRITER.lock().new_line();
}
