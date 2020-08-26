#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate cortex_m_semihosting;
use cortex_m_semihosting::debug;

pub use cortex_m_semihosting::hprint as print;
pub use cortex_m_semihosting::hprintln as println;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>()).unwrap();
        self();
        println!("[ok]").unwrap();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len()).unwrap();
    for test in tests {
        test.run();
    }
    println!("Done.").unwrap();
    debug::exit(debug::EXIT_SUCCESS);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n").unwrap();
    println!("Error: {}\n", info).unwrap();
    debug::exit(debug::EXIT_FAILURE);
    loop {}
}