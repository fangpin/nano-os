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
    os::serial_println!("[failed]\n");
    os::serial_println!("Error: {}\n", info);
    os::exit_qemu(os::QemuExitCode::Success);
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

    os::init();
    // invoke the breakpoint instruction
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("didn't crash");
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
