//! Quadrature encoder interface (QEI) example with hardware index-pulse reset.
//!
//! On each index pulse the timer counter is automatically reset to zero by
//! hardware (via TIMx_ECR).  An interrupt flag is also set and logged.
//!
//! TODO: assign the correct timer and pins for your board.
//! TODO: set `ab_position` to match the A/B logic levels at your encoder's
//!       index pulse — check the encoder datasheet timing diagram.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::timer::qei::{AdvancedConfig, Config, Etp, Etps, Fidx, FilterValue, Idir, IndexConfig, IndexPosition, QeiMode, Qei};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("QEI encoder with index pulse example");

    let index_config = IndexConfig {
        // Reset counter on index pulse regardless of counting direction.
        direction: Idir::Both,
        // Keep resetting on every index pulse (not just the first).
        first_only: Fidx::AlwaysActive,
        // TODO: set this to the A/B state that coincides with your index pulse.
        ab_position: IndexPosition::A0B0,
        etr_polarity: Etp::NotInverted,
        etr_filter: FilterValue::NoFilter,
        etr_prescaler: Etps::Div1,
        etr_pull: embassy_stm32::gpio::Pull::None,
    };

    let config = AdvancedConfig {
        base: Config {
            mode: QeiMode::Mode3,
            ..Default::default()
        },
        index: Some(index_config),
        // Enable the index-event interrupt so we can detect resets in software.
        enable_index_interrupt: true,
        enable_direction_change_interrupt: false,
    };

    // TODO: replace TIM4, PB6 (CH1), PB7 (CH2), PA8 (ETR/index) with the
    // timer and pins wired to your encoder's A, B, and index outputs.
    let qei = Qei::new_advanced_with_index(p.TIM4, p.PB6, p.PB7, p.PA8, config);

    loop {
        if qei.index_event_pending() {
            info!("Index pulse detected — counter reset to zero");
            qei.clear_index_event();
        }

        let count = qei.count();
        let direction = match qei.read_direction() {
            embassy_stm32::timer::qei::Direction::Upcounting => "up",
            embassy_stm32::timer::qei::Direction::Downcounting => "down",
        };
        info!("count={} direction={}", count, direction);

        Timer::after_millis(100).await;
    }
}
