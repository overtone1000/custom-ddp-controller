use std::{error::Error, sync::Arc};

use angular_units::Deg;
use ddp_rs::connection::DDPConnection;

use crate::pixels::{pixel::HSV, pixelstrip::PixelStrip};

pub fn red_green_blue(
    mut conn: DDPConnection,
    mut pixels: PixelStrip,
) -> Result<(), Box<dyn Error>> {
    // this crate is non blocking, so with out the sleep, it will send them all instantly

    let red: HSV = HSV::new(Deg(0.0), 1.0, 1.0);
    let green: HSV = HSV::new(Deg(120.0), 1.0, 1.0);
    let blue: HSV = HSV::new(Deg(240.0), 1.0, 1.0);

    let colors = [red, green, blue];
    let mut indices = [0, 1];

    for _ in 0..5 {
        for i in 0..pixels.count() {
            for a in 0..i {
                pixels.set_pixel_hsv(a, colors[indices[1]]);
            }
            for a in i..pixels.count() {
                pixels.set_pixel_hsv(a, colors[indices[0]]);
            }

            pixels.flush_and_write(&mut conn)?;

            std::thread::sleep(std::time::Duration::from_millis(10));
            // this crate is non blocking, so with out the sleep, it will send them all instantly
        }

        for i in 0..indices.len() {
            indices[i] = indices[i] + 1;
            if indices[i] > 2 {
                indices[i] = 0;
            }
        }
    }

    Ok(())
}

pub fn hue_progression(
    mut conn: DDPConnection,
    mut pixels: PixelStrip,
) -> Result<(), Box<dyn Error>> {
    // this crate is non blocking, so with out the sleep, it will send them all instantly

    for _ in 0..5 {
        for i in 0..360 {
            let hue = HSV::new(Deg(f64::from(i)), 1.0, 1.0);

            for a in 0..pixels.count() {
                pixels.set_pixel_hsv(a, hue);
            }

            pixels.flush_and_write(&mut conn)?;

            std::thread::sleep(std::time::Duration::from_millis(10));
            // this crate is non blocking, so with out the sleep, it will send them all instantly
        }
    }

    Ok(())
}

pub fn rainbow_oscillation(
    mut conn: DDPConnection,
    mut pixels: PixelStrip,
) -> Result<(), Box<dyn Error>> {
    // this crate is non blocking, so with out the sleep, it will send them all instantly

    let start = std::time::SystemTime::now();
    let rotation_frequency: f64 = 1.0; //Hz

    let pixel_count_f64 = f64::from(pixels.count() as u32);

    loop {
        let time = std::time::SystemTime::elapsed(&start);

        match time {
            Ok(elapsed) => {
                let rotational_offset_from_time = elapsed.as_secs_f64() * rotation_frequency;

                for a in 0..pixels.count() {
                    let rotational_offset_from_location = f64::from(a as u32) / pixel_count_f64;
                    let total_offset =
                        rotational_offset_from_location + rotational_offset_from_time;
                    let total_offset = total_offset % 1.0; //This prevents a bug with large angles.
                    let final_angle = Deg(360.0 * total_offset);

                    let hue = HSV::new(final_angle, 1.0, 1.0);

                    pixels.set_pixel_hsv(a, hue);
                }

                pixels.flush_and_write(&mut conn)?;

                std::thread::sleep(std::time::Duration::from_millis(10));
                // this crate is non blocking, so with out the sleep, it will send them all instantly
            }
            Err(e) => {
                eprintln!("Couldn't get time. {:?}", e);
            }
        }
    }

    Ok(())
}
