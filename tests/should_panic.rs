#![no_std]
#![no_main]

use core::panic::PanicInfo;
use tinyx86::{QemuExitCode, exit_qemu, serial_print, serial_println};


/*    main    */
#[no_mangle]
pub extern "C" fn _start() -> ! {
	test_should_fail();
	serial_println!("[NO PANIC]");
	exit_qemu(QemuExitCode::Failure);
	
	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);

	loop {}
}

/**************/

/*    test (no harness)   */
fn test_should_fail() {
	serial_print!("should_panic::test_should_fail...\t");
	assert_eq!(0, 1);
}
/***************************/
