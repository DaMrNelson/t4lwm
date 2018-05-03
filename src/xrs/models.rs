#[derive(Debug)]
pub enum BitOrder {LeastSignificant, MostSignificant} // 0,1
#[derive(Debug)]
pub enum ByteOrder {LSBFirst, MSBFirst} // 0,1
#[derive(Debug)]
pub enum ScreenBackingStores {Never, WhenMapped, Always} // 0,1,2
#[derive(Debug)]
pub enum VisualType {StaticGray, GrayScale, StaticColor, PseudoColor, TrueColor, DirectColor} // 0,1,2,3,4,5

#[derive(Debug)]
pub struct ConnectInfo {
    pub status_code: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub additional_data_len: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub max_request_length: u16,
    pub num_screens: u8,
    pub num_formats: u8,
    pub image_byte_order: ByteOrder,
    pub bitmap_format_bit_order: BitOrder,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: char,
    pub max_keycode: char,
    pub vendor: String,
    pub formats: Vec<Format>,
    pub screens: Vec<Screen>
}

impl ConnectInfo {
    pub fn empty() -> ConnectInfo {
        ConnectInfo {
            status_code: 0,
            protocol_major_version: 0,
            protocol_minor_version: 0,
            additional_data_len: 0,
            release_number: 0,
            resource_id_base: 0,
            resource_id_mask: 0,
            motion_buffer_size: 0,
            max_request_length: 0,
            num_screens: 0,
            num_formats: 0,
            image_byte_order: ByteOrder::LSBFirst,
            bitmap_format_bit_order: BitOrder::LeastSignificant,
            bitmap_format_scanline_unit: 0,
            bitmap_format_scanline_pad: 0,
            min_keycode: 0 as char,
            max_keycode: 0 as char,
            vendor: String::new(),
            formats: vec![],
            screens: vec![]
        }
    }
}

#[derive(Debug)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8
}

impl Format {
    pub fn empty() -> Format {
        Format {
            depth: 0,
            bits_per_pixel: 0,
            scanline_pad: 0
        }
    }
}

#[derive(Debug)]
pub struct Screen {
    pub root: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32, // TODO: This sets SETOfEVENT, but I don't know where the spec for this is
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: ScreenBackingStores,
    pub save_unders: bool,
    pub root_depth: u8,
    pub num_depths: u8,
    pub depths: Vec<Depth>
}

impl Screen {
    pub fn empty() -> Screen {
        Screen {
            root: 0,
            default_colormap: 0,
            white_pixel: 0,
            black_pixel: 0,
            current_input_masks: 0,
            width_in_pixels: 0,
            height_in_pixels: 0,
            width_in_millimeters: 0,
            height_in_millimeters: 0,
            min_installed_maps: 0,
            max_installed_maps: 0,
            root_visual: 0,
            backing_stores: ScreenBackingStores::Never,
            save_unders: false,
            root_depth: 0,
            num_depths: 0,
            depths: vec![]
        }
    }
}

#[derive(Debug)]
pub struct Depth {
    pub depth: u8,
    pub num_visuals: u16,
    pub visuals: Vec<Visual>
}

impl Depth {
    pub fn empty() -> Depth {
        Depth {
            depth: 0,
            num_visuals: 0,
            visuals: vec![]
        }
    }
}

#[derive(Debug)]
pub struct Visual {
    pub id: u32,
    pub class: VisualType,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32
}

impl Visual {
    pub fn empty() -> Visual {
        Visual {
            id: 0,
            class: VisualType::StaticGray,
            bits_per_rgb_value: 0,
            colormap_entries: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0
        }
    }
}
