#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4::stm32f429;

fn delay(mut n: u32) {
    while n != 0 {
        cortex_m::asm::nop();
        n -= 1;
    }
}


#[entry]
fn main() -> ! {
    let peripherals = stm32f429::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;
    rcc.ahb1enr.write(|w| w.gpiogen().set_bit());

    let gpiog = &peripherals.GPIOG;
    gpiog.moder.write(|w| w.moder13().bits(0b01).moder14().bits(0b01));

    loop {
        gpiog.bsrr.write(|w| w.bs13().set_bit());
        delay(200000);
        gpiog.bsrr.write(|w| w.br13().set_bit());
        delay(200000);

        gpiog.bsrr.write(|w| w.bs14().set_bit());
        delay(200000);
        gpiog.bsrr.write(|w| w.br14().set_bit());
        delay(200000);
    }
}
