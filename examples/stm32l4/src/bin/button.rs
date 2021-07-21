#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[path = "../example_common.rs"]
mod example_common;
use cortex_m_rt::entry;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::pac;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use example_common::*;

#[entry]
fn main() -> ! {
    info!("Hello World!");

    unsafe {
        pac::DBGMCU.cr().modify(|w| {
            w.set_dbg_sleep(true);
            w.set_dbg_standby(true);
            w.set_dbg_stop(true);
        });

        pac::RCC.apb2enr().modify(|w| {
            w.set_syscfgen(true);
        });

        pac::RCC.ahb2enr().modify(|w| {
            w.set_gpioaen(true);
            w.set_gpioben(true);
            w.set_gpiocen(true);
            w.set_gpioden(true);
            w.set_gpioeen(true);
            w.set_gpiofen(true);
        });
    }

    let p = embassy_stm32::init(Default::default());

    let button = Input::new(p.PC13, Pull::Up);
    let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        if button.is_high().unwrap() {
            info!("high");
            led1.set_high().unwrap();
            led2.set_low().unwrap();
        } else {
            info!("low");
            led1.set_low().unwrap();
            led2.set_high().unwrap();
        }
    }
}
