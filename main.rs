#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt as rt;

use cc1312::Peripherals;
use rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return

#[entry]
fn main() -> ! {
    loop {

        let p = Peripherals::take().unwrap();
        // application logic
    }
}
