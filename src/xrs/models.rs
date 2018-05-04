#[derive(Debug)]
pub enum BitOrder {LeastSignificant, MostSignificant} // 0,1
#[derive(Debug)]
pub enum ByteOrder {LSBFirst, MSBFirst} // 0,1

#[derive(Debug)]
pub enum Event {KeyPress, KeyRelease, OwnerGrabButton, ButtonPress, ButtonRelease, EnterWindow, LeaveWindow, PointerMotion, PointerMotionHint, Button1Motion, Button2Motion, Button3Motion, Button4Motion, Button5Motion, ButtonMotion, Exposure, VisibilityChange, StructureNotify, ResizeRedirect, SubstructureNotify, SubstructureRedirect, FocusChange, PropertyChange, ColormapChange, KeymapState}
        // TODO: Find actual EVENT info in spec and map it properly
#[derive(Debug)]
pub enum DeviceEvent {KeyPress, KeyRelease, ButtonPress, ButtonRelease, PointerMotion, Button1Motion, Button2Motion, Button3Motion, Button4Motion, Button5Motion, ButtonMotion}
        // TODO: Find actual DEVICEEVENT info in spec and map it properly

#[derive(Debug)]
pub enum ScreenBackingStores {Never, WhenMapped, Always} // 0,1,2
#[derive(Debug)]
pub enum VisualType {StaticGray, GrayScale, StaticColor, PseudoColor, TrueColor, DirectColor} // 0,1,2,3,4,5

#[derive(Debug)]
pub enum WindowInputType {CopyFromParent, InputOutput, InputOnly} // 0,1,2
#[derive(Debug)]
pub enum WindowValueBackgroundPixmap {None, ParentRelative} // 0,1
#[derive(Debug)]
pub enum WindowValueBorderPixmap {None} // 0
#[derive(Debug)]
pub enum WindowValueBackingStore {NotUseful, WhenMapped, Always} // 0,1,2

// TODO: Properly map
#[derive(Debug)]
pub enum BitGravity {Forget, Static, NorthWest, North, NorthEast, West, Center, East, SouthWest, South, SouthEast}
impl BitGravity {
    pub fn get_value(&self) -> u8 {
        match self {
            &BitGravity::Forget => 0,
            &BitGravity::Static => 1,
            &BitGravity::NorthWest => 2,
            &BitGravity::North => 3,
            &BitGravity::NorthEast => 4,
            &BitGravity::West => 5,
            &BitGravity::Center => 6,
            &BitGravity::East => 7,
            &BitGravity::SouthWest => 8,
            &BitGravity::South => 9,
            &BitGravity::SouthEast => 10
        }
    }
}

// TODO: Properly map
#[derive(Debug)]
pub enum WindowGravity {Unmap, Static, NorthWest, North, NorthEast, West, Center, East, SouthWest, South, SouthEast}
impl WindowGravity {
    pub fn get_value(&self) -> u8 {
        match self {
            &WindowGravity::Unmap => 0,
            &WindowGravity::Static => 1,
            &WindowGravity::NorthWest => 2,
            &WindowGravity::North => 3,
            &WindowGravity::NorthEast => 4,
            &WindowGravity::West => 5,
            &WindowGravity::Center => 6,
            &WindowGravity::East => 7,
            &WindowGravity::SouthWest => 8,
            &WindowGravity::South => 9,
            &WindowGravity::SouthEast => 10
        }
    }
}

pub const WINDOW_BITMASK_BACKGROUND_PIXMAP: u32 = 0x00000001;
pub const WINDOW_BITMASK_BACKGROUND_PIXEL: u32 = 0x00000002;
pub const WINDOW_BITMASK_BORDER_PIXMAP: u32 = 0x00000004;
pub const WINDOW_BITMASK_BORDER_PIXEL: u32 = 0x00000008;
pub const WINDOW_BITMASK_BIT_GRAVITY: u32 = 0x00000010;
pub const WINDOW_BITMASK_WIN_GRAVITY: u32 = 0x00000020;
pub const WINDOW_BITMASK_BACKING_STORE: u32 = 0x00000040;
pub const WINDOW_BITMASK_BACKING_PLANES: u32 = 0x00000080;
pub const WINDOW_BITMASK_BACKING_PIXEL: u32 = 0x00000100;
pub const WINDOW_BITMASK_OVERRIDE_REDIRECT: u32 = 0x00000200;
pub const WINDOW_BITMASK_SAVE_UNDER: u32 = 0x00000400;
pub const WINDOW_BITMASK_EVENT_MASK: u32 = 0x00000800;
pub const WINDOW_BITMASK_DO_NOT_PROPAGATE_MASK: u32 = 0x00001000;
pub const WINDOW_BITMASK_COLORMAP: u32 = 0x00002000;
pub const WINDOW_BITMASK_CURSOR: u32 = 0x00004000;

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

#[derive(Debug)]
pub struct Window {
    pub depth: u8,
    pub wid: u32, // Window's ID
    pub parent: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: WindowInputType,
    pub visual_id: u32,
    pub bitmask: u32,
    pub values: Vec<WindowValue>
}

#[derive(Debug)]
pub struct WindowValue {
    pub background_pixmap: u32, // 0 = None, 1 = Parent Relative // TODO: Enum that?
    pub background_pixel: u32,
    pub border_pixmap: u32, // 0 = CopyFromParent // TODO: Enum that?
    pub border_pixel: u32,
    pub bit_gravity: BitGravity,
    pub win_gravity: WindowGravity,
    pub backing_store: WindowValueBackingStore,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: bool,
    pub save_under: bool,
    pub event_mask: u32,
    pub do_not_propagate_mask: u32,
    pub colormap: u32, // 0 = CopyFromParent // TODO: Enum that?
    pub cursor: u32 // 0 = None // TODO: Enum that?
}
