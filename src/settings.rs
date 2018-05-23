extern crate xrb;

use xrb::models::Color;

pub struct Settings {
    pub background_color: Color,

    pub win_bg: Color,
    pub win_border_width: u16,
    pub win_border_color: Color,
    pub win_title_bg: Color,
    pub win_title_fg: Color,
    pub win_title_border_width: u16,
    pub win_title_border_color: Color
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            background_color: Color::from_num(0x000000),

            win_bg: Color::from_num(0x000000),
            win_border_width: 1,
            win_border_color: Color::from_num(0x000000),
            win_title_bg: Color::from_num(0x333333),
            win_title_fg: Color::from_num(0xEEEEEE),
            win_title_border_width: 1,
            win_title_border_color: Color::from_num(0x000000)
        }
    }
}
