use std::time::{Duration, Instant};

use curtain::CurtainModifier;

use crate::{
    BACK_LEFT_CORNER, BACK_MIDDLE, FRONT_LEFT_CORNER, FRONT_MIDDLE, FRONT_RIGHT_CORNER, LAST_LED,
    LED_COUNT, LEFT_MIDDLE, RIGHT_MIDDLE, START_LED,
};

use super::PixelModifier;

pub mod curtain;

enum FadeoutOption {
    CornerCurtain,
    SideCurtain,
}

const ALL_FADEOUT_OPTIONS: [FadeoutOption; 2] =
    [FadeoutOption::CornerCurtain, FadeoutOption::SideCurtain];

pub fn random_fadeout() -> PixelModifier {
    let selected_option = &ALL_FADEOUT_OPTIONS[rand::random_range(0..ALL_FADEOUT_OPTIONS.len())];

    //let selected_option = FadeoutOption::SideCurtain;

    match selected_option {
        FadeoutOption::CornerCurtain => PixelModifier::Curtain(CurtainModifier::new(
            Instant::now(),
            Duration::from_secs(7),
            50.0,
            vec![
                (RIGHT_MIDDLE, START_LED),
                (RIGHT_MIDDLE, FRONT_RIGHT_CORNER),
                (FRONT_MIDDLE, FRONT_RIGHT_CORNER),
                (FRONT_MIDDLE, FRONT_LEFT_CORNER),
                (LEFT_MIDDLE, FRONT_LEFT_CORNER),
                (LEFT_MIDDLE, BACK_LEFT_CORNER),
                (BACK_MIDDLE, BACK_LEFT_CORNER),
                (BACK_MIDDLE, LAST_LED),
            ],
        )),
        FadeoutOption::SideCurtain => PixelModifier::Curtain(CurtainModifier::new(
            Instant::now(),
            Duration::from_secs(7),
            50.0,
            vec![
                (START_LED, RIGHT_MIDDLE),
                (FRONT_RIGHT_CORNER, RIGHT_MIDDLE),
                (FRONT_RIGHT_CORNER, FRONT_MIDDLE),
                (FRONT_LEFT_CORNER, FRONT_MIDDLE),
                (FRONT_LEFT_CORNER, LEFT_MIDDLE),
                (BACK_LEFT_CORNER, LEFT_MIDDLE),
                (BACK_LEFT_CORNER, BACK_MIDDLE),
                (LAST_LED, BACK_MIDDLE),
            ],
        )),
    }
}
