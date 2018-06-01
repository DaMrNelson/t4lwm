extern crate xrb;

use xrb::models::{Color, KeyButton};

pub struct Settings {
    pub mod_key: KeyButton,
    pub background_color: Color,

    pub win_bg: Color,
    pub win_bg_focused: Color,

    pub win_border_width_left: u16,
    pub win_border_width_top: u16,
    pub win_border_width_right: u16,
    pub win_border_width_bottom: u16,
    pub win_border_color: Color,
    pub win_border_color_focused: Color,

    pub win_title_bg: Color,
    pub win_title_fg: Color,
    pub win_title_bg_focused: Color,
    pub win_title_fg_focused: Color,

    pub win_title_border_width_left: u16,
    pub win_title_border_width_top: u16,
    pub win_title_border_width_right: u16,
    pub win_title_border_width_bottom: u16,
    pub win_title_border_color: Color,
    pub win_title_border_color_focused: Color
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            mod_key: KeyButton::Mod4,
            background_color: Color::from_num(0x444444),

            win_bg: Color::from_num(0x000000),
            win_bg_focused: Color::from_num(0x000000),

            win_border_width_left: 0,
            win_border_width_top: 0,
            win_border_width_right: 0,
            win_border_width_bottom: 0,
            win_border_color: Color::from_num(0x000000),
            win_border_color_focused: Color::from_num(0x000000),

            win_title_bg: Color::from_num(0x5F676A),
            win_title_fg: Color::from_num(0xFFFFFF),
            win_title_bg_focused: Color::from_num(0x111111),
            win_title_fg_focused: Color::from_num(0xFFFFFF),

            win_title_border_width_left: 1,
            win_title_border_width_top: 1,
            win_title_border_width_right: 1,
            win_title_border_width_bottom: 1,
            //win_title_border_color: Color::from_num(0x000000),
            win_title_border_color: Color::from_num(0xFF0000),
            //win_title_border_color_focused: Color::from_num(0x666666)
            win_title_border_color_focused: Color::from_num(0x00FF00)
        }
    }
}
