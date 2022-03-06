#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	#[cfg(test)]
	test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("running {} tests", tests.len());
	for test in tests {
		test();
	}
}
