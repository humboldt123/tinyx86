#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tinyx86::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tinyx86::println;

/*    main    */
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("hello, world{}", "!");

	#[cfg(test)]
	test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	tinyx86::test_panic_handler(info)
}
/**************/


/*    qemu interface    */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}
/************************/
