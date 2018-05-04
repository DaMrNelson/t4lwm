use xrs::xwriter::XBufferedWriter;

// Root trait for all values (ie GraphicsContextValue)
pub trait Value {
    fn get_mask(&self) -> u32;
    fn write<T: XBufferedWriter>(&self, client: &mut T);
}

// Root trait for all valued types
pub trait Valued {
    fn val(&self) -> u32;
}

////////////////////////////////////////
/// XRB TYPES
////////////////////////////////////////


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


////////////////////////////////////////
/// X TYPES
////////////////////////////////////////


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
    pub values: Vec<WindowValue>
}

#[derive(Debug)]
pub struct Depth {
    pub depth: u8,
    pub num_visuals: u16,
    pub visuals: Vec<Visual>
}

#[derive(Debug)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8
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

#[derive(Debug)]
pub struct Pixmap {
    pub depth: u8,
    pub pid: u32, // Pixmap's ID
    pub drawable: u32, // Window or Pixmap ID
    pub width: u16,
    pub height: u16
}

#[derive(Debug)]
pub struct GraphicsContext {
    pub cid: u32, // Graphic Context ID
    pub drawable: u32, // Window or Pixmap ID
    pub values: Vec<GraphicsContextValue>
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

impl Depth {
    pub fn empty() -> Depth {
        Depth {
            depth: 0,
            num_visuals: 0,
            visuals: vec![]
        }
    }
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


////////////////////////////////////////
/// VALUED
////////////////////////////////////////


#[derive(Debug)]
pub enum BitOrder {
    LeastSignificant,
    MostSignificant
}
impl Valued for BitOrder {
    fn val(&self) -> u32 {
        match self {
            &BitOrder::LeastSignificant => 0,
            &BitOrder::MostSignificant => 1
        }
    }
}

#[derive(Debug)]
pub enum ByteOrder {
    LSBFirst = 0,
    MSBFirst = 1
}
impl Valued for ByteOrder {
    fn val(&self) -> u32 {
        match self {
            &ByteOrder::LSBFirst => 0,
            &ByteOrder::MSBFirst => 1
        }
    }
}

#[derive(Debug)]
pub enum Event {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState,
    Exposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    OwnerGrabButton
}
impl Valued for Event {
    fn val(&self) -> u32 {
        match self {
            &Event::KeyPress => 0x00000001,
            &Event::KeyRelease => 0x00000002,
            &Event::ButtonPress => 0x00000004,
            &Event::ButtonRelease => 0x00000008,
            &Event::EnterWindow => 0x00000010,
            &Event::LeaveWindow => 0x00000020,
            &Event::PointerMotion => 0x00000040,
            &Event::PointerMotionHint => 0x00000080,
            &Event::Button1Motion => 0x00000100,
            &Event::Button2Motion => 0x00000200,
            &Event::Button3Motion => 0x00000400,
            &Event::Button4Motion => 0x00000800,
            &Event::Button5Motion => 0x00001000,
            &Event::ButtonMotion => 0x00002000,
            &Event::KeymapState => 0x00004000,
            &Event::Exposure => 0x00008000,
            &Event::VisibilityChange => 0x00010000,
            &Event::StructureNotify => 0x00020000,
            &Event::ResizeRedirect => 0x00040000,
            &Event::SubstructureNotify => 0x00080000,
            &Event::SubstructureRedirect => 0x00100000,
            &Event::FocusChange => 0x00200000,
            &Event::PropertyChange => 0x00400000,
            &Event::ColormapChange => 0x00800000,
            &Event::OwnerGrabButton => 0x01000000
        }
    }
}

#[derive(Debug)]
pub enum PointerEvent {
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState
}
impl Valued for PointerEvent {
    fn val(&self) -> u32 {
        match self {
            &PointerEvent::ButtonPress => 0x00000004,
            &PointerEvent::ButtonRelease => 0x00000008,
            &PointerEvent::EnterWindow => 0x00000010,
            &PointerEvent::LeaveWindow => 0x00000020,
            &PointerEvent::PointerMotion => 0x00000040,
            &PointerEvent::PointerMotionHint => 0x00000080,
            &PointerEvent::Button1Motion => 0x00000100,
            &PointerEvent::Button2Motion => 0x00000200,
            &PointerEvent::Button3Motion => 0x00000400,
            &PointerEvent::Button4Motion => 0x00000800,
            &PointerEvent::Button5Motion => 0x00001000,
            &PointerEvent::ButtonMotion => 0x00002000,
            &PointerEvent::KeymapState => 0x00004000
        }
    }
}

#[derive(Debug)]
pub enum DeviceEvent {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    PointerMotion,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion
}
impl Valued for DeviceEvent {
    fn val(&self) -> u32 {
        match self {
            &DeviceEvent::KeyPress => 0x00000001,
            &DeviceEvent::KeyRelease => 0x00000002,
            &DeviceEvent::ButtonPress => 0x00000004,
            &DeviceEvent::ButtonRelease => 0x00000008,
            &DeviceEvent::PointerMotion => 0x00000040,
            &DeviceEvent::Button1Motion => 0x00000100,
            &DeviceEvent::Button2Motion => 0x00000200,
            &DeviceEvent::Button3Motion => 0x00000400,
            &DeviceEvent::Button4Motion => 0x00000800,
            &DeviceEvent::Button5Motion => 0x00001000,
            &DeviceEvent::ButtonMotion => 0x00002000
        }
    }
}

#[derive(Debug)]
pub enum ScreenBackingStores {
    Never,
    WhenMapped,
    Always
}
impl Valued for ScreenBackingStores {
    fn val(&self) -> u32 {
        match self {
            &ScreenBackingStores::Never => 0,
            &ScreenBackingStores::WhenMapped => 1,
            &ScreenBackingStores::Always => 2
        }
    }
}

#[derive(Debug)]
pub enum VisualType {
    StaticGray,
    GrayScale,
    StaticColor,
    PseudoColor,
    TrueColor,
    DirectColor
}
impl Valued for VisualType {
    fn val(&self) -> u32 {
        match self {
            &VisualType::StaticGray => 0,
            &VisualType::GrayScale => 1,
            &VisualType::StaticColor => 2,
            &VisualType::PseudoColor => 3,
            &VisualType::TrueColor => 4,
            &VisualType::DirectColor => 5
        }
    }
}

#[derive(Debug)]
pub enum WindowInputType {
    CopyFromParent,
    InputOutput,
    InputOnly
}
impl Valued for WindowInputType {
    fn val(&self) -> u32 {
        match self {
            &WindowInputType::CopyFromParent => 0,
            &WindowInputType::InputOutput => 1,
            &WindowInputType::InputOnly => 2
        }
    }
}

#[derive(Debug)]
pub enum WindowValueBackingStore {
    NotUseful,
    WhenMapped,
    Always
}
impl Valued for WindowValueBackingStore {
    fn val(&self) -> u32 {
        match self {
            &WindowValueBackingStore::NotUseful => 0,
            &WindowValueBackingStore::WhenMapped => 1,
            &WindowValueBackingStore::Always => 2
        }
    }
}

#[derive(Debug)]
pub enum BitGravity {
	Forget,
	Static,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast
}
impl Valued for BitGravity {
    fn val(&self) -> u32 {
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

#[derive(Debug)]
pub enum WindowGravity {
	Unmap,
	Static,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast
}
impl Valued for WindowGravity {
    fn val(&self) -> u32 {
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

#[derive(Debug)]
pub enum GCFunction {
	Clear,
	And,
	AndReverse,
	Copy,
	AndInverted,
	NoOp,
	Xor,
	Or,
	Nor,
	Equiv,
	Invert,
	OrReverse,
	CopyInverted,
	OrInverted,
	Nand,
	Set
}
impl Valued for GCFunction {
    fn val(&self) -> u32 {
        match self {
            &GCFunction::Clear => 0,
            &GCFunction::And => 1,
            &GCFunction::AndReverse => 2,
            &GCFunction::Copy => 3,
            &GCFunction::AndInverted => 4,
            &GCFunction::NoOp => 5,
            &GCFunction::Xor => 6,
            &GCFunction::Or => 7,
            &GCFunction::Nor => 8,
            &GCFunction::Equiv => 9,
            &GCFunction::Invert => 10,
            &GCFunction::OrReverse => 11,
            &GCFunction::CopyInverted => 12,
            &GCFunction::OrInverted => 13,
            &GCFunction::Nand => 14,
            &GCFunction::Set => 15
        }
    }
}

#[derive(Debug)]
pub enum GCLineStyle {
	Solid,
	OnOffDash,
	DoubleDash
}
impl Valued for GCLineStyle {
    fn val(&self) -> u32 {
        match self {
            &GCLineStyle::Solid => 0,
            &GCLineStyle::OnOffDash => 1,
            &GCLineStyle::DoubleDash => 2
        }
    }
}

#[derive(Debug)]
pub enum GCCapStyle {
	NotLast,
	Butt,
	Round,
	Projecting
}
impl Valued for GCCapStyle {
    fn val(&self) -> u32 {
        match self {
            &GCCapStyle::NotLast => 0,
            &GCCapStyle::Butt => 1,
            &GCCapStyle::Round => 2,
            &GCCapStyle::Projecting => 3
        }
    }
}

#[derive(Debug)]
pub enum GCJoinStyle {
	Miter,
	Round,
	Bevel
}
impl Valued for GCJoinStyle {
    fn val(&self) -> u32 {
        match self {
            &GCJoinStyle::Miter => 0,
            &GCJoinStyle::Round => 1,
            &GCJoinStyle::Bevel => 2
        }
    }
}

#[derive(Debug)]
pub enum GCFillStyle {
	Solid,
	Tiled,
	Stippled,
	OpaqueStippled
}
impl Valued for GCFillStyle {
    fn val(&self) -> u32 {
        match self {
            &GCFillStyle::Solid => 0,
            &GCFillStyle::Tiled => 1,
            &GCFillStyle::Stippled => 2,
            &GCFillStyle::OpaqueStippled => 3
        }
    }
}

#[derive(Debug)]
pub enum GCFillRule {
	EvenOdd,
	Winding
}
impl Valued for GCFillRule {
    fn val(&self) -> u32 {
        match self {
            &GCFillRule::EvenOdd => 0,
        	&GCFillRule::Winding => 1
        }
    }
}

#[derive(Debug)]
pub enum GCSubWindowMode {
	ClipByChildren = 0,
	IncludeInferiors = 1
}
impl Valued for GCSubWindowMode {
    fn val(&self) -> u32 {
        match self {
            &GCSubWindowMode::ClipByChildren => 0,
	        &GCSubWindowMode::IncludeInferiors => 1
        }
    }
}

#[derive(Debug)]
pub enum GCArcMode {
	Chord,
	PieSlice
}
impl Valued for GCArcMode {
    fn val(&self) -> u32 {
        match self {
            &GCArcMode::Chord => 0,
	        &GCArcMode::PieSlice => 1
        }
    }
}


////////////////////////////////////////
/// VALUES
////////////////////////////////////////


#[derive(Debug)]
pub enum WindowValue {
    BackgroundPixmap(u32),
    BackgroundPixel(u32),
    BorderPixmap(u32),
    BorderPixel(u32),
    BitGravity(BitGravity),
    WinGravity(WindowGravity),
    BackingStore(WindowValueBackingStore),
    BackingPlanes(u32),
    BackingPixel(u32),
    OverrideRedirect(bool),
    SaveUnder(bool),
    EventMask(u32),
    DoNotPropagateMask(u32),
    Colormap(u32),
    Cursor(u32)
}

#[derive(Debug)]
pub enum GraphicsContextValue {
    Function(GCFunction),
    PlaneMask(u32),
    Foreground(u32),
    Background(u32),
    LineWidth(u16),
    LineStyle(GCLineStyle),
    CapStyle(GCCapStyle),
    JoinStyle(GCJoinStyle),
    FillStyle(GCFillStyle),
    FillRule(GCFillRule),
    Tile(u32), // pixmap ID
    Stipple(u32), // pixmap ID
    TileStippleXOrigin(u16),
    TileStippleYOrigin(u16),
    Font(u32),
    SubWindowMode(GCSubWindowMode),
    GraphicsExposures(bool),
    ClipXOrigin(u16),
    ClipYOrigin(u16),
    ClipMask(u32), // pixmap ID
    DashOffset(u16),
    Dashes(u8),
    ArcMode(GCArcMode)
}


////////////////////////////////////////
/// VALUES METHODS
////////////////////////////////////////


impl Value for WindowValue {
    fn get_mask(&self) -> u32 {
        match self {
            &WindowValue::BackgroundPixmap(_) => 0x00000001,
            &WindowValue::BackgroundPixel(_) => 0x00000002,
            &WindowValue::BorderPixmap(_) => 0x00000004,
            &WindowValue::BorderPixel(_) => 0x00000008,
            &WindowValue::BitGravity(_) => 0x00000010,
            &WindowValue::WinGravity(_) => 0x00000020,
            &WindowValue::BackingStore(_) => 0x00000040,
            &WindowValue::BackingPlanes(_) => 0x00000080,
            &WindowValue::BackingPixel(_) => 0x00000100,
            &WindowValue::OverrideRedirect(_) => 0x00000200,
            &WindowValue::SaveUnder(_) => 0x00000400,
            &WindowValue::EventMask(_) => 0x00000800,
            &WindowValue::DoNotPropagateMask(_) => 0x00001000,
            &WindowValue::Colormap(_) => 0x00002000,
            &WindowValue::Cursor(_) => 0x00004000
        }
    }

    fn write<T: XBufferedWriter>(&self, client: &mut T) {
        match self {
            &WindowValue::BackgroundPixmap(val) => client.write_val_u32(val),
            &WindowValue::BackgroundPixel(val) => client.write_val_u32(val),
            &WindowValue::BorderPixmap(val) => client.write_val_u32(val),
            &WindowValue::BorderPixel(val) => client.write_val_u32(val),
            &WindowValue::BitGravity(ref val) => client.write_val(val.val()),
            &WindowValue::WinGravity(ref val) => client.write_val(val.val()),
            &WindowValue::BackingStore(ref val) => client.write_val(val.val()),
            &WindowValue::BackingPlanes(val) => client.write_val_u32(val),
            &WindowValue::BackingPixel(val) => client.write_val_u32(val),
            &WindowValue::OverrideRedirect(val) => client.write_val_bool(val),
            &WindowValue::SaveUnder(val) => client.write_val_bool(val),
            &WindowValue::EventMask(val) => client.write_val_u32(val),
            &WindowValue::DoNotPropagateMask(val) => client.write_val_u32(val),
            &WindowValue::Colormap(val) => client.write_val_u32(val),
            &WindowValue::Cursor(val) => client.write_val_u32(val)
        }
    }
}

impl Value for GraphicsContextValue {
    fn get_mask(&self) -> u32 {
        match self {
            &GraphicsContextValue::Function(_) => 0x00000001,
            &GraphicsContextValue::PlaneMask(_) => 0x00000002,
            &GraphicsContextValue::Foreground(_) => 0x00000004,
            &GraphicsContextValue::Background(_) => 0x00000008,
            &GraphicsContextValue::LineWidth(_) => 0x00000010,
            &GraphicsContextValue::LineStyle(_) => 0x00000020,
            &GraphicsContextValue::CapStyle(_) => 0x00000040,
            &GraphicsContextValue::JoinStyle(_) => 0x00000080,
            &GraphicsContextValue::FillStyle(_) => 0x00000100,
            &GraphicsContextValue::FillRule(_) => 0x00000200,
            &GraphicsContextValue::Tile(_) => 0x00000400,
            &GraphicsContextValue::Stipple(_) => 0x00000800,
            &GraphicsContextValue::TileStippleXOrigin(_) => 0x00001000,
            &GraphicsContextValue::TileStippleYOrigin(_) => 0x00002000,
            &GraphicsContextValue::Font(_) => 0x00004000,
            &GraphicsContextValue::SubWindowMode(_) => 0x00008000,
            &GraphicsContextValue::GraphicsExposures(_) => 0x00010000,
            &GraphicsContextValue::ClipXOrigin(_) => 0x00020000,
            &GraphicsContextValue::ClipYOrigin(_) => 0x00040000,
            &GraphicsContextValue::ClipMask(_) => 0x00080000,
            &GraphicsContextValue::DashOffset(_) => 0x00100000,
            &GraphicsContextValue::Dashes(_) => 0x00200000,
            &GraphicsContextValue::ArcMode(_) => 0x00400000
        }
    }

    fn write<T: XBufferedWriter>(&self, client: &mut T) {
        match self {
            &GraphicsContextValue::Function(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::PlaneMask(val) => client.write_val_u32(val),
            &GraphicsContextValue::Foreground(val) => client.write_val_u32(val),
            &GraphicsContextValue::Background(val) => client.write_val_u32(val),
            &GraphicsContextValue::LineWidth(val) => client.write_val_u16(val),
            &GraphicsContextValue::LineStyle(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::CapStyle(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::JoinStyle(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::FillStyle(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::FillRule(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::Tile(val) => client.write_val_u32(val),
            &GraphicsContextValue::Stipple(val) => client.write_val_u32(val),
            &GraphicsContextValue::TileStippleXOrigin(val) => client.write_val_u16(val),
            &GraphicsContextValue::TileStippleYOrigin(val) => client.write_val_u16(val),
            &GraphicsContextValue::Font(val) => client.write_val_u32(val),
            &GraphicsContextValue::SubWindowMode(ref val) => client.write_val(val.val()),
            &GraphicsContextValue::GraphicsExposures(val) => client.write_val_bool(val),
            &GraphicsContextValue::ClipXOrigin(val) => client.write_val_u16(val),
            &GraphicsContextValue::ClipYOrigin(val) => client.write_val_u16(val),
            &GraphicsContextValue::ClipMask(val) => client.write_val_u32(val),
            &GraphicsContextValue::DashOffset(val) => client.write_val_u16(val),
            &GraphicsContextValue::Dashes(val) => client.write_val_u8(val),
            &GraphicsContextValue::ArcMode(ref val) => client.write_val(val.val())
        };
    }
}
