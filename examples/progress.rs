extern crate xencelabs_quick_keys;

extern crate hidapi;

use hidapi::HidApi;

use xencelabs_quick_keys::*;

fn run(api: HidApi) -> QKResult<()> {
    let mut progress = 0;
    
    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            dev.set_screen_orientation(ScreenOrientation::Rotate270)?;
            dev.set_screen_brightness(ScreenBrightness::Medium)?;
            dev.set_wheel_speed(WheelSpeed::Faster)?;
            dev.set_ring_color(0, 0, 0)?;
            loop {
                match dev.read() {
                    Ok(ev) => match ev {
                        Event::Wheel { direction: WheelDirection::Left } => {
                            progress = if progress > 0 { progress - 1 } else { 0 };
                            dev.show_overlay_text(format!("[{:_<30}]", "=".repeat(progress*30/100).to_string()).as_str(), 1)
                        },
                        Event::Wheel { direction: WheelDirection::Right } => {
                            progress = if progress < 100 { progress + 1 } else { 100 };
                            dev.show_overlay_text(format!("[{:_<30}]", "=".repeat(progress*30/100).to_string()).as_str(), 1)
                        },
                        _ => Ok(()),
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
