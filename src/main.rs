#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
	let mut thing1 = 1;
	let mut thing2 = 2;
	
	let bob: *mut usize = 0xdeadbeef as *mut usize;
	unsafe { *bob = 5; }
    loop
	{
		unsafe { *bob = *bob + 1; }
		unsafe { core::ptr::read_volatile(&thing1) };
		thing1 = thing1 + 1;
		thing2 = thing2 + 1;
		asm::nop();
    }
}