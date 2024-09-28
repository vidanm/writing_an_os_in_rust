#![no_std]
#![no_main]
#![feature(custom_test_frameworks)] // Nécessaire car le module test est défini dans le standard
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //Le test runner va chercher à s'éxecuter dans un main

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
