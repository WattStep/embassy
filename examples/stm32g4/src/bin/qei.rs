//! Quadrature encoder interface (QEI) example without index pulse.
//!
//! Reads the encoder count and direction every 100 ms and logs them.
//!
//! TODO: assign the correct timer and pins for your board.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::peripherals;
use embassy_stm32::timer::qei::{Config, Qei, QeiMode};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("QEI encoder example");

    // TODO: replace TIM4, PB6 (CH1), PB7 (CH2) with the timer and pins wired
    // to your encoder's A and B outputs.
    let config = Config {
        mode: QeiMode::Mode3, // count on both edges of both channels
        ..Default::default()
    };
    let qei = Qei::new(p.TIM4, p.PB6, p.PB7, config);

    loop {
        let count = qei.count();
        let direction = match qei.read_direction() {
            embassy_stm32::timer::qei::Direction::Upcounting => "up",
            embassy_stm32::timer::qei::Direction::Downcounting => "down",
        };
        info!("count={} direction={}", count, direction);

        Timer::after_millis(100).await;
    }
}
