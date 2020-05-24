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
    let peripherals = Peripherals::take().unwrap();
        // application logic
    let gpio = peripherals.GPIO; //calling GPIO

    let led1 = peripherals
    .GPIO.dout7_4.read().dio6().bits(); //calling LED

    let led2 = peripherals
    .GPIO.dout7_4.read().dio7().bits(); //calling LED

    let read1 = peripherals.GPIO.evflags31_0
    .read().dio6();// event reg. for LED

    let read2 = peripherals.GPIO.evflags31_0
    .read().dio7();// event reg. for LED
    
   

    loop{
        // let seconds = aon.sec.read().bits();
        // let subseconds = aon.subsec.read().bits(); 
        
        // if seconds >= 3{
        //     let x = 0;
        //     aon.ctl.modify(| r, w|{
        //         w.reset().set_bit()
        //     });
        //     while aon.ctl.read().reset().bit_is_set(){
        //         continue;
          //  }
        if read1.bit_is_set(){
            !led1;
        }

        if read2.bit_is_set(){
            !led2;
        }

        
        continue;
    }
    

    loop {
        timer.delay_ms(100);
        if gpio.dout7_4.read().bits() == 0 {
        gpio.dout7_4.write(|w| w.set_bit());
            } else {
                gpio.dout7_4.write(|w| w.clear_bit());
            }
    }
}

}
