pub mod pixels;
pub mod services;

pub const LED_COUNT: usize = 450; //There are 450 LEDs. This was confirmed.

pub const START_LED: usize = 0;
pub const LAST_LED: usize = LED_COUNT - 1;

pub const DISPLAY_FREQUENCY: f64 = 60.0; //Hz 60Hz seems pretty good! Doesn't seem to jitter or overload

pub const RIGHT_MIDDLE: usize = 74;
pub const FRONT_RIGHT_CORNER: usize = 148;
pub const FRONT_MIDDLE: usize = 186;
pub const FRONT_LEFT_CORNER: usize = 224;
pub const LEFT_MIDDLE: usize = 298;
pub const BACK_LEFT_CORNER: usize = 372;
pub const BACK_MIDDLE: usize = 410;
