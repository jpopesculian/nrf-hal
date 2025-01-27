// Required connections:
//
// - P0.28 <-> P0.29

#![deny(warnings)]
#![no_std]
#![no_main]

use defmt_rtt as _;
use nrf52840_hal as _;
use panic_probe as _;

use nrf52840_hal::gpio::{Floating, Input, OpenDrain, Output, Pin};

struct State {
    input_pin: Pin<Input<Floating>>,
    output_pin: Pin<Output<OpenDrain>>,
}

#[defmt_test::tests]
mod tests {
    use defmt::{assert, unwrap};
    use nrf52840_hal::{
        gpio::{p0, Level, OpenDrainConfig},
        pac,
        prelude::*,
    };

    use super::State;

    #[init]
    fn init() -> State {
        let p = unwrap!(pac::Peripherals::take());
        let port0 = p0::Parts::new(p.P0);

        let input_pin = port0.p0_28.into_floating_input().degrade();
        let output_pin = port0
            .p0_29
            .into_open_drain_output(OpenDrainConfig::Standard0Disconnect1, Level::High)
            .degrade();

        State {
            input_pin,
            output_pin,
        }
    }

    #[test]
    fn set_low_is_low(state: &mut State) {
        state.output_pin.set_low().unwrap();
        assert!(state.input_pin.is_low().unwrap());
    }

    // with the current API we cannot test this w/o an _external_ pull-up
    /*
    #[test]
    fn set_high_is_high(state: &mut State) {
        state.output_pin.set_high().unwrap();
        assert!(state.input_pin.is_high().unwrap());
    }
    */
}
