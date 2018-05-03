#![allow(dead_code)]
// TODO: Don't actually allow dead code

mod protocol;
mod models;

extern crate bufstream;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::io::BufReader;
use self::bufstream::BufStream;

use self::models::*;

pub struct XClient {
    connected: bool,
    connect_info: ConnectInfo,
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
            let mut connect_req = [0u8; 12];
            connect_req[0] = protocol::CONNECT_LSB;
            connect_req[1] = 0;
            self.write_u16(&mut connect_req, 2, protocol::CONNECT_MAJOR);
            self.write_u16(&mut connect_req, 4, protocol::CONNECT_MINOR);
            self.write_u16(&mut connect_req, 6, 0);
            self.write_u16(&mut connect_req, 8, 0);
            connect_req[10] = 0;
            connect_req[11] = 0;
            self.buf.write_all(&connect_req).unwrap();
            self.buf.flush().unwrap();
        }

        // Read response header
        {
            // Read the head
            self.connect_info.status_code = self.read_u8();
            self.read_ignore(1);
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
            self.read_ignore(4);

            self.connect_info.vendor = self.read_str(vendor_length as usize);
            self.read_ignore((vendor_length as usize) % 4);
            println!("Server Vendor: {}", self.connect_info.vendor);

            // Formats (8 bytes each)
            for _ in 0..self.connect_info.num_formats {
                let mut format = Format::empty();
                format.depth = self.read_u8();
                format.bits_per_pixel = self.read_u8();
                format.scanline_pad = self.read_u8();
                self.read_ignore(5);

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
                screen.save_unders = match self.read_u8() {
                    0 => false,
                    1 => true,
                    val => panic!("Unexpected save unders {}", val)
                };
                screen.root_depth = self.read_u8();
                screen.num_depths = self.read_u8();

                // Read depths (x bytes, where x is a multiple of 4)
                for _ in 0..screen.num_depths {
                    let mut depth = Depth::empty();
                    depth.depth = self.read_u8();
                    self.read_ignore(1);
                    depth.num_visuals = self.read_u16();
                    self.read_ignore(4); // Unused
                    
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
                        self.read_ignore(4); // Unused

                        depth.visuals.push(visual);
                    }

                    screen.depths.push(depth);
                }

                self.connect_info.screens.push(screen);
            }
        }
    }

    /**
     * Writes a u16 to the given array starting at start (and obviously ending two bytes later).
     * Expects little endian.
     */
    fn write_u16(&self, arr: &mut [u8], start: usize, input: u16) {
        arr[start] = input as u8;
        arr[start + 1] = (input >> 8) as u8;
    }

    /**
     * Reads X bytes and ignores them.
     */
    fn read_ignore(&mut self, len: usize) {
        self.buf.consume(len);
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
