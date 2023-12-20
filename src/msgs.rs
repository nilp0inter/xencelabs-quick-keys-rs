/// Pad the rightmost part of some array with zeroes up to given length.
fn pad_zeroes<const A: usize, const B: usize>(arr: [u8; A]) -> [u8; B] {
    assert!(B >= A);
    let mut b = [0; B];
    b[..A].copy_from_slice(&arr);
    b
}

#[cfg(test)]
mod tests_pad_zeroes {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_on_incorrect_size() {
        let _result: [u8; 2] = pad_zeroes([1, 2, 3]);
    }

    #[test]
    fn it_doesnt_change_full_arrays() {
        let result: [u8; 3] = pad_zeroes([1, 2, 3]);
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn it_should_fill_with_zeroes() {
        let result: [u8; 5] = pad_zeroes([1, 2, 3]);
        assert_eq!(result, [1, 2, 3, 0, 0]);
    }
}

//
// OUTPUT MESSAGES
//

/// A message to subscribe to key events
pub fn msg_subscribe_to_key_events() -> [u8; 32] {
    pad_zeroes([0x02, 0xb0, 0x04])
}

/// A message to subscribe to battery changes
pub fn msg_subscribe_to_battery() -> [u8; 32] {
    pad_zeroes([0x02, 0xb4, 0x10])
}

/// Possible device screen orientations
pub enum ScreenOrientation {
    Rotate0 = 1,
    Rotate90 = 2,
    Rotate180 = 3,
    Rotate270 = 4,
}

/// A message to rotate the screen
pub fn msg_rotate_screen(rot: ScreenOrientation) -> [u8; 32] {
    pad_zeroes([0x02, 0xb1, rot as u8])
}

/// Possible screen brightness levels
pub enum ScreenBrightness {
    Off = 0,
    Low = 1,
    Medium = 2,
    Full = 3,
}

/// A message to change the screen brightness level
pub fn msg_set_screen_brightness(level: ScreenBrightness) -> [u8; 32] {
    pad_zeroes([0x02, 0xb1, 0x0a, 0x01, level as u8])
}

/// Possible wheel speed settings
pub enum WheelSpeed {
    Slowest = 5,
    Slower = 4,
    Normal = 3,
    Faster = 2,
    Fastest = 1,
}

/// A message to change the wheel speed
pub fn msg_set_wheel_speed(speed: WheelSpeed) -> [u8; 32] {
    pad_zeroes([0x02, 0xb4, 0x04, 0x01, 0x01, speed as u8])
}

/// A message to set for how long the device would be awake (after losing connection)
pub fn msg_set_sleep_timeout(minutes: u8) -> [u8; 32] {
    pad_zeroes([0x02, 0xb4, 0x08, 0x01, minutes])
}

/// A message to set the wheel outer ring led color
pub fn msg_set_wheel_color(r: u8, g: u8, b: u8) -> [u8; 32] {
    pad_zeroes([0x02, 0xb4, 0x01, 0x01, 0x00, 0x00, r, g, b])
}

/// A message to set the text on a given key
// TODO: investigate how to set text longer than 8 chars
pub fn msg_set_key_text(key: u8, text: &str) -> [u8; 32] {
    let mut body = [0u8; 32];
    body[..6].clone_from_slice(&[
        0x02,
        0xb1,
        0x00,
        key + 1,
        0x00,
        (if text.len() <= 8 { text.len() * 2 } else { 16 }) as u8,
    ]);

    let mut payload = text
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect::<Vec<u8>>();
    payload.resize(16, 0);
    body[16..].clone_from_slice(&payload);
    body
}

/// Part of a message sequence to show a text overlay
fn submsg_overlay_chunk(is_cont: bool, duration: u8, text: &str, has_more: bool) -> [u8; 32] {
    let mut body = [0u8; 32];
    body[..7].clone_from_slice(&[
        0x02,
        0xb1,
        if is_cont { 0x06 } else { 0x05 },
        duration,
        0x00,
        (if text.len() <= 8 { text.len() * 2 } else { 16 }) as u8,
        has_more as u8,
    ]);

    let mut payload = text
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect::<Vec<u8>>();
    payload.resize(16, 0);
    body[16..].clone_from_slice(&payload);
    body
}

/// A message sequence to show a text overlay
// TODO: consider unicode problems
pub fn msgs_show_overlay_text(duration: u8, text: &str) -> Vec<[u8; 32]> {
    assert!(text.len() <= 32);
    let mut res = Vec::new();
    for (i, chunk, has_more) in text
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .enumerate()
        .map(|(i, w)| {
            (
                i,
                w,
                i == ((text.len() / 8) - if (text.len() % 8) == 0 { 2 } else { 1 }),
            )
        })
    {
        res.push(submsg_overlay_chunk(
            i != 0,
            duration,
            &chunk,
            i==1 && text.len() > 16 || i > 0 && has_more,
        ))
    }
    res
}

/// This test suite matches primarily the data obtained from the source code of the
/// node-xencelabs-quick-keys library.
#[cfg(test)]
mod tests_output_msgs {
    use super::*;

    #[test]
    fn it_should_match_subscribe_to_events() {
        let result = msg_subscribe_to_key_events();
        assert_eq!(
            result,
            [
                2, 176, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn it_should_match_subscribe_to_battery() {
        let result = msg_subscribe_to_battery();
        assert_eq!(
            result,
            [
                2, 180, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn it_should_match_rotate_screen() {
        let result = msg_rotate_screen(ScreenOrientation::Rotate90);
        assert_eq!(
            result,
            [
                2, 177, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn it_should_match_set_screen_brightness() {
        let result = msg_set_screen_brightness(ScreenBrightness::Medium);
        assert_eq!(
            result,
            [
                2, 177, 10, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn it_should_match_set_wheel_speed() {
        let result = msg_set_wheel_speed(WheelSpeed::Faster);
        assert_eq!(
            result,
            [
                2, 180, 4, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    // TODO: Extract from api
    //    #[test]
    //    fn it_should_match_set_sleep_timeout() {
    //        let result = msg_set_wheel_speed(WheelSpeed::Faster);
    //        assert_eq!(result, [ 2, 180, 4, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ])
    //    }

    #[test]
    fn it_should_match_set_wheel_color() {
        let result = msg_set_wheel_color(1, 2, 3);
        assert_eq!(
            result,
            [
                2, 180, 1, 1, 0, 0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn it_should_match_set_key_text() {
        let result = msg_set_key_text(3, "baazquux");
        assert_eq!(
            result,
            [
                2, 177, 0, 4, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 97, 0, 97, 0, 122, 0,
                113, 0, 117, 0, 117, 0, 120, 0
            ]
        )
    }

    #[test]
    fn it_should_match_show_overlay_text_multiple_of_eight() {
        let result = msgs_show_overlay_text(42, "Is this real life? <=0=>");
        assert_eq!(
            result,
            vec![
                [
                    2, 177, 5, 42, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 115, 0, 32, 0, 116,
                    0, 104, 0, 105, 0, 115, 0, 32, 0
                ],
                [
                    2, 177, 6, 42, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 0, 101, 0, 97, 0, 108,
                    0, 32, 0, 108, 0, 105, 0, 102, 0
                ],
                [
                    2, 177, 6, 42, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 63, 0, 32, 0, 60,
                    0, 61, 0, 48, 0, 61, 0, 62, 0
                ],
            ]
        )
    }

    #[test]
    fn it_should_match_show_overlay_text_non_multiple_of_eight() {
        let result = msgs_show_overlay_text(42, "Is this real life?");
        assert_eq!(
            result,
            vec![
                [
                    2, 177, 5, 42, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 115, 0, 32, 0, 116,
                    0, 104, 0, 105, 0, 115, 0, 32, 0
                ],
                [
                    2, 177, 6, 42, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 0, 101, 0, 97, 0, 108,
                    0, 32, 0, 108, 0, 105, 0, 102, 0
                ],
                [
                    2, 177, 6, 42, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 63, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0
                ],
            ]
        )
    }

    #[test]
    fn it_should_match_show_overlay_text_broken() {
        let result = msgs_show_overlay_text(2, "Disco, disco!");
        assert_eq!(
            result,
            vec![
                [
                    2, 177, 5, 2, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 105, 0, 115, 0, 99,
                    0, 111, 0, 44, 0, 32, 0, 100, 0
                ],
                [
                    2, 177, 6, 2, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 115, 0, 99, 0, 111,
                    0, 33, 0, 0, 0, 0, 0, 0, 0
                ],
            ]
        )
    }

    #[test]
    fn it_should_match_show_overlay_text_progress() {
        let result = msgs_show_overlay_text(2, "[//////////////////////////////]");
        assert_eq!(
            result,
            vec![
                [ 2, 177, 5, 2, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 91, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0 ],
                [ 2, 177, 6, 2, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0 ],
                [ 2, 177, 6, 2, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0 ],
                [ 2, 177, 6, 2, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 47, 0, 93, 0 ],
            ]
        )
    }
}

//
// INPUT MESSAGES
//

/// Represent the direction of movement of the wheel
#[derive(Debug, PartialEq, Clone)]
pub enum WheelDirection {
    Right,
    Left,
}

/// The state of the buttons at any given moment (true => press, false => not press)
#[derive(Debug, PartialEq, Clone)]
pub struct ButtonState {
    pub button_0: bool,
    pub button_1: bool,
    pub button_2: bool,
    pub button_3: bool,
    pub button_4: bool,
    pub button_5: bool,
    pub button_6: bool,
    pub button_7: bool,
    pub button_extra: bool,
    pub button_wheel: bool,
}

/// Represent a state change of the device
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Button { state: ButtonState },
    Wheel { direction: WheelDirection },
    Battery { percent: u8 },
    Unknown { data: [u8; 10] },
}

/// Process an input message from the device and translates it to an Event
/// For messages that are malformed or not yet understood it returns Unknown
pub fn process_input(data: &[u8; 10]) -> Event {
    if data[0] == 0x02 {
        if data[1] == 0xf0 {
            let wheel_byte = data[7];
            if wheel_byte & 0x01 > 0 {
                Event::Wheel {
                    direction: WheelDirection::Right,
                }
            } else if wheel_byte & 0x02 > 0 {
                Event::Wheel {
                    direction: WheelDirection::Left,
                }
            } else {
                let keys1 = data[2];
                let keys2 = data[3];
                Event::Button {
                    state: ButtonState {
                        button_0: keys1 & (1 << 0) > 0,
                        button_1: keys1 & (1 << 1) > 0,
                        button_2: keys1 & (1 << 2) > 0,
                        button_3: keys1 & (1 << 3) > 0,
                        button_4: keys1 & (1 << 4) > 0,
                        button_5: keys1 & (1 << 5) > 0,
                        button_6: keys1 & (1 << 6) > 0,
                        button_7: keys1 & (1 << 7) > 0,
                        button_extra: keys2 & (1 << 0) > 0,
                        button_wheel: keys2 & (1 << 1) > 0,
                    },
                }
            }
        } else if data[1] == 0xf2 && data[2] == 0x01 {
            Event::Battery { percent: data[3] }
        } else {
            Event::Unknown { data: *data }
        }
    } else {
        Event::Unknown { data: *data }
    }
}

#[cfg(test)]
mod tests_input_msgs {
    use super::*;

    #[test]
    fn it_should_decode_wheel_left() {
        let result = process_input(&pad_zeroes([2, 240, 0, 0, 0, 0, 0, 2, 0, 0]));
        assert_eq!(
            result,
            Event::Wheel {
                direction: WheelDirection::Left
            }
        )
    }

    #[test]
    fn it_should_decode_wheel_right() {
        let result = process_input(&pad_zeroes([2, 240, 0, 0, 0, 0, 0, 1, 0, 0]));
        assert_eq!(
            result,
            Event::Wheel {
                direction: WheelDirection::Right
            }
        )
    }

    #[test]
    fn it_should_decode_no_button_press() {
        let result = process_input(&pad_zeroes([2, 240, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_wheel_button_press() {
        let result = process_input(&pad_zeroes([2, 240, 0, 2, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: true,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_zero_press() {
        let result = process_input(&pad_zeroes([2, 240, 1, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: true,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_one_press() {
        let result = process_input(&pad_zeroes([2, 240, 2, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: true,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_two_press() {
        let result = process_input(&pad_zeroes([2, 240, 4, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: true,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_three_press() {
        let result = process_input(&pad_zeroes([2, 240, 8, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: true,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_four_press() {
        let result = process_input(&pad_zeroes([2, 240, 16, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: true,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_five_press() {
        let result = process_input(&pad_zeroes([2, 240, 32, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: true,
                    button_6: false,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_six_press() {
        let result = process_input(&pad_zeroes([2, 240, 64, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: true,
                    button_7: false,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_seven_press() {
        let result = process_input(&pad_zeroes([2, 240, 128, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: true,
                    button_extra: false,
                    button_wheel: false,
                }
            }
        )
    }

    #[test]
    fn it_should_decode_button_extra_press() {
        let result = process_input(&pad_zeroes([2, 240, 0, 1, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            result,
            Event::Button {
                state: ButtonState {
                    button_0: false,
                    button_1: false,
                    button_2: false,
                    button_3: false,
                    button_4: false,
                    button_5: false,
                    button_6: false,
                    button_7: false,
                    button_extra: true,
                    button_wheel: false,
                }
            }
        )
    }
}
