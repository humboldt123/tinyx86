#![no_std]
#![no_main]

static HELLO_WORLD: &[u8] = b"hello, world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	let vga_mem = 0xb8000 as *mut u8;

	// print hello world in 80x25 char vga memory
	for (i, &byte) in HELLO_WORLD.iter().enumerate() {
		unsafe {
			*vga_mem.offset(i as isize * 2)		= byte;
			*vga_mem.offset(i as isize * 2 + 1)	= 0xb;		// light cyan
		}
	}

	loop {}
}

#[panic_handler]
fn panic(__info: &core::panic::PanicInfo) -> ! {
	loop {}
}
