#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[path = "../example_common.rs"]
mod example_common;
use core::fmt::Write;
use embassy::executor::Executor;
use embassy::time::Clock;
use embassy::util::Forever;
use embassy_stm32::dbgmcu::Dbgmcu;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_traits::uart::Write as _Write;
use example_common::*;

use hal::prelude::*;
use stm32h7xx_hal as hal;

use cortex_m_rt::entry;
use heapless::String;
use stm32h7::stm32h743 as pac;

#[embassy::task]
async fn main_task() {
    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let mut usart = Uart::new(p.UART7, p.PF6, p.PF7, p.DMA1_CH0, NoDma, config);

    for n in 0u32.. {
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        usart.write(s.as_bytes()).await.ok();

        info!("wrote DMA");
    }
}

struct ZeroClock;

impl Clock for ZeroClock {
    fn now(&self) -> u64 {
        0
    }
}

static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let pp = pac::Peripherals::take().unwrap();

    let pwrcfg = pp.PWR.constrain().freeze();

    let rcc = pp.RCC.constrain();

    rcc.sys_ck(96.mhz())
        .pclk1(48.mhz())
        .pclk2(48.mhz())
        .pclk3(48.mhz())
        .pclk4(48.mhz())
        .pll1_q_ck(48.mhz())
        .freeze(pwrcfg, &pp.SYSCFG);

    unsafe {
        Dbgmcu::enable_all();
    }

    unsafe { embassy::time::set_clock(&ZeroClock) };

    let executor = EXECUTOR.put(Executor::new());

    executor.run(|spawner| {
        unwrap!(spawner.spawn(main_task()));
    })
}
