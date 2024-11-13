#![no_std]
#![no_main]

pub mod config;
// pub mod errors;
// pub mod heap;

// use crate::errors::Error;

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_00, Level::Low, OutputDrive::Standard);

    let config_softsim_static_profile = "01120829430512308000010214985437000000020810100320C131BF48444CC6DF74953FE8B0CCF3ED04205A2BBC5EB5FE0634092D0E35AF2184F005201B49D1343ED67141DD25FA8E8FAFBE8606208F05401CF36E5667574FD9EA705E4F970808373631330b1031303336323332310a2034333437363535333630393336373339";
    defmt::info!("{}", config_softsim_static_profile);

    loop {
        led.set_high();
        defmt::info!("high");
        Timer::after_millis(250).await;
        led.set_low();
        defmt::info!("low");
        Timer::after_millis(5000).await;
    }
}

// write an nrf9151 softsim apdu command interface
