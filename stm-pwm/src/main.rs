#![no_main]
#![no_std]

use panic_halt as _;
use fugit::ExtU32;
use cortex_m_rt::entry;
use stm32f1xx_hal as hal;
use hal::{pac, prelude::*, timer::{Channel, Tim2NoRemap}};

#[entry]

fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut afio = p.AFIO.constrain(&mut rcc);
    let mut gpioa = p.GPIOA.split(&mut rcc);
    let mut delay = p.TIM1.delay::<1_000_000>(&mut rcc);

    let c1 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);


    let mut pwm = p.TIM2.pwm_hz::<Tim2NoRemap, _, _>(c1,&mut afio.mapr, 1.kHz(), &mut rcc);
    pwm.enable(Channel::C2);

    let max = pwm.get_max_duty();

    loop {
        for i in 0..=max {
            pwm.set_duty(Channel::C2, i);
            delay.delay(300.micros());
        }
        for i in (0..max).rev() {
            pwm.set_duty(Channel::C2, i);
            delay.delay(300.micros());
        }
    }
}