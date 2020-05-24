#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt as rt;

use cc1312::Peripherals;
use rt::entry;
use cortex_m::asm::delay;
// use `main` as the entry point of this application
// `main` is not allowed to return

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
        // application logic
    
        // PRCM::pdctl0::periph_on - Needs to be set to one/set_bit
peripherals.PRCM.pdctl0.modify(|_r,w|{
    w.periph_on().set_bit();
    w

});
        // PRCM::gpioclkgr::clk_en - Needs to be set to one/set_bit
peripherals.PRCM.gpioclkgr.modify(|_r,w|{
    w.clk_en().set_bit();
    w
});
        // PRCM::clkloadctl::load - Needs to be set to one/set_bit
peripherals.PRCM.clkloadctl.modify(|_r,w|{
    w.load().set_bit();
    w
});
        // WAIT, until PRCM::clkloadct::load_done IS 1/bit_is_se
       loop {
        
        let done = peripherals.PRCM.clkloadctl.read().load_done().bit_is_set();
        
        if done { 
            break;
        }
       } 
//    let peripherals.GPIO.dout7_4.write(|w|)
// So your steps are basically:

// Use IOCFG6::PORT_ID to set pin 6 to be a GPIO (write zero/clear bit)
 peripherals.IOC.iocfg6.modify(|_r,w|{
    unsafe{ w.port_id().bits(0)};
    // Use IOCFG6::IE to set pin 6 to NOT be an input (write zero/clear bit)
     w.ie().clear_bit();
     w
 });


// Use DOE31_0::DIO6 to set pin 6 to be an output (write 1/set bit)
peripherals.GPIO.doe31_0.modify(|_r,w|{
    w.dio6().set_bit();
    w

});
// Set the GPIO output pin high or low by setting or clearing DOUT7_4::DIO6 high or low (write 1/set bit OR write zero/clear bit)
    
       
    loop {
        delay(1_000_000);
       peripherals.GPIO.dout7_4.modify(|_r,w|{
           w.dio6().set_bit();
           w

       });
       delay(1_000_000);
       peripherals.GPIO.dout7_4.modify(|_r,w|{
        w.dio6().clear_bit();
        w
       });
    }
}

