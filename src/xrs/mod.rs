#![allow(dead_code)]
// TODO: Don't actually allow dead code

pub mod protocol;
pub mod models;

extern crate bufstream;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use self::bufstream::BufStream;

use self::models::*;

pub struct XClient {
    pub connected: bool,
    pub connect_info: ConnectInfo,
    buf: BufStream<UnixStream>,
    buf_one_byte: Vec<u8>,
    buf_two_byte: Vec<u8>,
    buf_four_byte: Vec<u8>
}

impl XClient {
    pub fn new(host: String) -> XClient {
        let stream = UnixStream::connect(host).unwrap();
        let mut client = XClient {
            connected: false,
            connect_info: ConnectInfo::empty(),
            buf: BufStream::new(stream),
            buf_one_byte: vec![0u8; 1],
            buf_two_byte: vec![0u8; 2],
            buf_four_byte: vec![0u8; 4]
        };
        
        return client;
    }

    /** Sends the connection parameters and returns if it connected or not. */
    pub fn connect(&mut self) {
        // Send connection string
        {
            self.write_u8(protocol::CONNECT_LSB);
            self.write_pad(1);
            self.write_u16(protocol::CONNECT_MAJOR);
            self.write_u16(protocol::CONNECT_MINOR);
            self.write_u16(0);
            self.write_u16(0);
            //self.write_pad(4); // Pad empty string
            //self.write_pad(4); // Pad empty string
            self.write_pad(1); // Pad empty string
            self.write_pad(1); // Pad empty string
            self.write_flush();
        }

        // Read response header
        {
            // Read the head
            self.connect_info.status_code = self.read_u8();
            self.read_pad(1);
            self.connect_info.protocol_major_version = self.read_u16();
            self.connect_info.protocol_minor_version = self.read_u16();
            self.connect_info.additional_data_len = self.read_u16(); // I have no idea why this exists. I cannot find the correlation between it and the actual length of the data sent.

            // Check if the connection was a success
            // TODO: Parse body of failures
            match self.connect_info.status_code {
                protocol::CONNECT_SUCCESS => (),
                protocol::CONNECT_FAILED => panic!("Got CONNECT_FAILED"),
                protocol::CONNECT_AUTHENTICATE => panic!("Got CONNECT_AUTHENTICATE"),
                code => panic!("Got unexpected value {}", code),
            };

            // Parse success info
            println!("Server Protocol: {}.{}", self.connect_info.protocol_major_version, self.connect_info.protocol_minor_version);
            self.connect_info.release_number = self.read_u32();
            self.connect_info.resource_id_base = self.read_u32();
            self.connect_info.resource_id_mask = self.read_u32();
            self.connect_info.motion_buffer_size = self.read_u32();
            let vendor_length = self.read_u16();
            self.connect_info.max_request_length = self.read_u16();
            self.connect_info.num_screens = self.read_u8();
            self.connect_info.num_formats = self.read_u8();
            self.connect_info.image_byte_order = match self.read_u8() {
                0 => ByteOrder::LSBFirst,
                1 => ByteOrder::MSBFirst,
                order => panic!("Unknown image byte order {}", order),
            };
            self.connect_info.bitmap_format_bit_order = match self.read_u8() {
                0 => BitOrder::LeastSignificant,
                1 => BitOrder::MostSignificant,
                order => panic!("Unknown bitmap format bit order {}", order)
            };
            self.connect_info.bitmap_format_scanline_unit = self.read_u8();
            self.connect_info.bitmap_format_scanline_pad = self.read_u8();
            self.connect_info.min_keycode = self.read_u8() as char;
            self.connect_info.max_keycode = self.read_u8() as char;
            self.read_pad(4);

            self.connect_info.vendor = self.read_str(vendor_length as usize);
            self.read_pad((vendor_length as usize) % 4);
            println!("Server Vendor: {}", self.connect_info.vendor);

            // Formats (8 bytes each)
            for _ in 0..self.connect_info.num_formats {
                let mut format = Format::empty();
                format.depth = self.read_u8();
                format.bits_per_pixel = self.read_u8();
                format.scanline_pad = self.read_u8();
                self.read_pad(5);

                self.connect_info.formats.push(format);
            }

            // Read screens (x bytes, where x is a multiple of 4)
            for _ in 0..self.connect_info.num_screens {
                let mut screen = Screen::empty();
                screen.root = self.read_u32();
                screen.default_colormap = self.read_u32();
                screen.white_pixel = self.read_u32();
                screen.black_pixel = self.read_u32();
                screen.current_input_masks = self.read_u32();
                screen.width_in_pixels = self.read_u16();
                screen.height_in_pixels = self.read_u16();
                screen.width_in_millimeters = self.read_u16();
                screen.height_in_millimeters = self.read_u16();
                screen.min_installed_maps = self.read_u16();
                screen.max_installed_maps = self.read_u16();
                screen.root_visual = self.read_u32();
                screen.backing_stores = match self.read_u8() {
                    0 => ScreenBackingStores::Never,
                    1 => ScreenBackingStores::WhenMapped,
                    2 => ScreenBackingStores::Always,
                    store => panic!("Unknown backing score {}", store)
                };
                screen.save_unders = self.read_bool();
                screen.root_depth = self.read_u8();
                screen.num_depths = self.read_u8();

                // Read depths (x bytes, where x is a multiple of 4)
                for _ in 0..screen.num_depths {
                    let mut depth = Depth::empty();
                    depth.depth = self.read_u8();
                    self.read_pad(1);
                    depth.num_visuals = self.read_u16();
                    self.read_pad(4); // Unused
                    
                    // Read visuals (24 x num visuals bytes)
                    for _ in 0..depth.num_visuals {
                        let mut visual = Visual::empty();
                        visual.id = self.read_u32();
                        visual.class = match self.read_u8() {
                            0 => VisualType::StaticGray,
                            1 => VisualType::GrayScale,
                            2 => VisualType::StaticColor,
                            3 => VisualType::PseudoColor,
                            4 => VisualType::TrueColor,
                            5 => VisualType::DirectColor,
                            class => panic!("Unknown visual class {}", class)
                        };
                        visual.bits_per_rgb_value = self.read_u8();
                        visual.colormap_entries = self.read_u16();
                        visual.red_mask = self.read_u32();
                        visual.green_mask = self.read_u32();
                        visual.blue_mask = self.read_u32();
                        self.read_pad(4); // Unused

                        depth.visuals.push(visual);
                    }

                    screen.depths.push(depth);
                }

                self.connect_info.screens.push(screen);
            }
        }
    }

    /** Tells the X Server to create a window */
    pub fn create_window(&mut self, window: &Window) {
        self.write_u8(protocol::OP_CREATE_WINDOW);
        self.write_u8(window.depth);
        self.write_u16(8 + window.values.len() as u16 * 12); // data length
        self.write_u32(window.wid);
        self.write_u32(window.parent);
        self.write_i16(window.x);
        self.write_i16(window.y);
        self.write_u16(window.width);
        self.write_u16(window.height);
        self.write_u16(window.border_width);
        self.write_u16(match window.class {
            WindowInputType::CopyFromParent => 0,
            WindowInputType::InputOutput => 1,
            WindowInputType::InputOnly => 2
        });
        self.write_u32(window.visual_id);
        self.write_u32(window.bitmask);
        
        for value in window.values.iter() {
            println!("Writing a window value!");
            self.write_u32(value.background_pixmap);
            self.write_u32(value.background_pixel);
            self.write_u32(value.border_pixmap);
            self.write_u32(value.border_pixel);
            self.write_u8(value.bit_gravity.get_value());
            self.write_u8(value.win_gravity.get_value());
            self.write_u8(match value.backing_store {
                WindowValueBackingStore::NotUseful => 0,
                WindowValueBackingStore::WhenMapped => 1,
                WindowValueBackingStore::Always => 2
            });
            self.write_u32(value.backing_planes);
            self.write_u32(value.backing_pixel);
            self.write_bool(value.override_redirect);
            self.write_bool(value.save_under);
            self.write_u32(value.event_mask);
            self.write_u32(value.do_not_propagate_mask);
            self.write_u32(value.colormap);
            self.write_u32(value.cursor);
            self.write_pad(3); // TODO: Do we actually write this pad? For some reason it is 25 bytes per value...
        }

        self.write_flush();
    }

    /** Flushes the buffer. */
    fn write_flush(&mut self) {
        self.buf.flush().unwrap();
    }

    /**
     * Writes X bytes (not guarnteed to be zero).
     */
    fn write_pad(&mut self, len: usize) {
        match len {
            1 => self.buf.write_all(&self.buf_one_byte),
            2 => self.buf.write_all(&self.buf_two_byte),
            4 => self.buf.write_all(&self.buf_four_byte),
            _ => self.buf.write_all(&vec![0u8; len])
        }.unwrap();
    }

    /**
     * Writes a bool to the buffer.
     */
    fn write_bool(&mut self, input: bool) {
        self.buf_one_byte[0] = match input {
            true => 1,
            false => 0
        };
        self.buf.write_all(&self.buf_one_byte).unwrap();
    }

    /**
     * Writes a u8 to the buffer.
     */
    fn write_u8(&mut self, input: u8) {
        self.buf_one_byte[0] = input;
        self.buf.write_all(&self.buf_one_byte).unwrap();
    }

    /**
     * Writes a i16 to the buffer.
     * Expects little endian.
     */
    fn write_i16(&mut self, input: i16) {
        self.write_u16(input as u16);
    }

    /**
     * Writes a u16 to the buffer.
     * Expects little endian.
     */
    fn write_u16(&mut self, input: u16) {
        self.buf_two_byte[0] = input as u8;
        self.buf_two_byte[1] = (input >> 8) as u8;
        self.buf.write_all(&self.buf_two_byte).unwrap();
    }

    /**
     * Writes a i32 to the buffer.
     * Expects little endian.
     */
    fn write_i32(&mut self, input: i32) {
        self.write_u32(input as u32);
    }

    /**
     * Writes a u32 to the buffer.
     * Expects little endian.
     */
    fn write_u32(&mut self, input: u32) {
        self.buf_four_byte[0] = input as u8;
        self.buf_four_byte[1] = (input >> 8) as u8;
        self.buf_four_byte[2] = (input >> 16) as u8;
        self.buf_four_byte[3] = (input >> 24) as u8;
        self.buf.write_all(&self.buf_four_byte).unwrap();
    }

    /**
     * Reads X bytes and ignores them.
     */
    fn read_pad(&mut self, len: usize) {
        self.buf.consume(len);
    }

    /**
     * Reads a bool from the buffer.
     */
    fn read_bool(&mut self) -> bool {
        self.buf.read_exact(&mut self.buf_one_byte).unwrap();
        match self.buf_one_byte[0] {
            1 => true,
            0 => false,
            other => panic!("Invalid integer for boolean: {}", other)
        }
    }

    /**
     * Reads a u8 from the buffer.
     */
    fn read_u8(&mut self) -> u8 {
        self.buf.read_exact(&mut self.buf_one_byte).unwrap();
        self.buf_one_byte[0]
    }

    /**
     * Reads a u16 from the buffer.
     * Expects little endian.
     */
    fn read_u16(&mut self) -> u16 {
        self.buf.read_exact(&mut self.buf_two_byte).unwrap();
        (self.buf_two_byte[0] as u16) + (self.buf_two_byte[1] as u16 >> 8)
    }

    /**
     * Reads a u32 from the buffer.
     * Expects little endian.
     */
    fn read_u32(&mut self) -> u32 {
        self.buf.read_exact(&mut self.buf_four_byte).unwrap();
        (self.buf_four_byte[0] as u32) + (self.buf_four_byte[1] as u32 >> 8) + (self.buf_four_byte[2] as u32 >> 12) + (self.buf_four_byte[3] as u32 >> 16)
    }

    /**
     * Reads a string from the buffer.
     */
    fn read_str(&mut self, len: usize) -> String {
        let mut buf = vec![0u8; len];
        self.buf.read_exact(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
