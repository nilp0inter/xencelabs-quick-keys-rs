extern crate xencelabs_quick_keys;

extern crate hidapi;

use std::{thread,time};
use hidapi::HidApi;

use xencelabs_quick_keys::*;

/// This function is the main entry point for the example. It opens a connection to the device,
/// and then sets the screen orientation, brightness, wheel speed, sleep timeout, ring color,
/// and key text. It then enters a loop, reading events from the device and setting the ring
/// color based on the wheel and buttons.
fn run(api: HidApi) -> QKResult<()> {
    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            dev.set_screen_orientation(ScreenOrientation::Rotate270)?;
            dev.set_screen_brightness(ScreenBrightness::Medium)?;
            dev.set_wheel_speed(WheelSpeed::Normal)?;
            dev.set_sleep_timeout(1)?;
            dev.set_ring_color(255, 255, 255)?;
            dev.set_key_text(0, "red")?;
            dev.set_key_text(1, "green")?;
            dev.set_key_text(2, "blue")?;
            dev.set_key_text(3, "yellow")?;
            dev.set_key_text(4, "purple")?;
            dev.set_key_text(5, "turquoise")?;
            dev.set_key_text(6, "white")?;
            dev.set_key_text(7, "off")?;
            thread::sleep(time::Duration::from_millis(1000));
            dev.show_overlay_text("Disco, disco!", 3)?;
            loop {
                match dev.read() {
                    Ok(ev) => match ev {
                        Event::Wheel { direction: WheelDirection::Left } => dev.set_ring_color(255, 0, 0),
                        Event::Wheel { direction: WheelDirection::Right } => dev.set_ring_color(0, 255, 0),
                        Event::Button { state: ButtonState { button_wheel: true, .. } } => dev.set_ring_color(0, 0, 255),
                        Event::Button { state: ButtonState { button_0: true, .. } } => dev.set_ring_color(255, 0, 0),
                        Event::Button { state: ButtonState { button_1: true, .. } } => dev.set_ring_color(0, 255, 0),
                        Event::Button { state: ButtonState { button_2: true, .. } } => dev.set_ring_color(0, 0, 255),
                        Event::Button { state: ButtonState { button_3: true, .. } } => dev.set_ring_color(255, 255, 0),
                        Event::Button { state: ButtonState { button_4: true, .. } } => dev.set_ring_color(255, 0, 255),
                        Event::Button { state: ButtonState { button_5: true, .. } } => dev.set_ring_color(0, 255, 255),
                        Event::Button { state: ButtonState { button_6: true, .. } } => dev.set_ring_color(255, 255, 255),
                        Event::Button { state: ButtonState { button_7: true, .. } } => dev.set_ring_color(0, 0, 0),
                        Event::Button { state: ButtonState { button_extra: true, .. } } => dev.show_overlay_text("Disco, disco!", 3),
                        Event::Button { state: ButtonState { .. } } => { println!("release"); Ok(()) },
                        Event::Unknown { data: d } => { println!("unknown! {:?}", d); Ok(()) },
                        Event::Battery { percent: p } => { println!("battery level: {:?}", p); Ok(()) },
                    },
                    Err(e) => Err(e),
                }?;
            }
        },
        Err(e) => { println!("Connection error!"); Err(e) },
    }
}

fn main() {
    match HidApi::new() {
        Ok(api) => match run(api) {
            Ok(_) => { println!("all good"); }
            Err(_) => { println!("error"); }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}
