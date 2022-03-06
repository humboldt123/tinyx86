#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
	let vga_mem = 0xb0000 as *mut u8;

	// print hello world in 80x25 char vga memory
	for (i, b) in b"hello, world".iter().enumerate() {
		unsafe {
			*vga_mem.offset(i as isize * 2)		= *b;
			*vga_mem.offset(i as isize * 2 + 1)	= 0x0f;
		}
	}

	loop {}
}

#[panic_handler]
fn panic(__info: &core::panic::PanicInfo) -> ! {
	loop {}
}
