#![no_std]

use libtock_platform::{ErrorCode, Syscalls};

/// The LEDs driver
///
/// # Example
/// ```ignore
/// use libtock2::Leds;
///
/// // Turn on led 0
/// let _ = Leds::on(0);
/// ```
pub struct Leds<S: Syscalls>(S);

impl<S: Syscalls> Leds<S> {
    /// Run a check against the leds capsule to ensure it is present.
    ///
    /// Returns `Ok(number_of_leds)` if the driver was present. This does not necessarily mean
    /// that the driver is working, as it may still fail to allocate grant
    /// memory.
    pub fn count() -> Result<u32, ErrorCode> {
        S::command(DRIVER_ID, LEDS_COUNT, 0, 0).to_result()
    }

    pub fn on(led: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_ID, LED_ON, led, 0).to_result()
    }

    pub fn off(led: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_ID, LED_OFF, led, 0).to_result()
    }

    pub fn toggle(led: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_ID, LED_TOGGLE, led, 0).to_result()
    }
}

const DRIVER_ID: u32 = 2;

// Command IDs
const LEDS_COUNT: u32 = 0;
const LED_ON: u32 = 1;
const LED_OFF: u32 = 2;
const LED_TOGGLE: u32 = 3;

#[cfg(test)]
mod tests;
