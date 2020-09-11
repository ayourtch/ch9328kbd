use raw_tty::IntoRawMode;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};
#[macro_use] extern crate log;

fn main() {
    env_logger::init();
    let devname = std::env::args().nth(1).unwrap_or("/dev/ttyUSB0".into());
    println!("Type here to send keystrokes to CH9328-emulated keyboard on {}.\n", &devname);
    println!("'sudo kill {}' to kill this process\n", std::process::id());
    let mut f = OpenOptions::new().write(true).open(&devname).unwrap();
    let mut stdin = stdin().into_raw_mode().unwrap();
    let mut character = [0];
    while let Ok(_) = stdin.read(&mut character) {
        let mut report = [0u8; 8];
        let report_off = [0u8; 8];
        let c0 = character[0];
        let (c, m) = match c0 {
            b'a'..=b'z' => (c0 - b'a' + 4, 0),
            b'A'..=b'Z' => (c0 - b'A' + 4, 0x02),
            b'1'..=b'9' => (c0 - b'1' + 30, 0),
            b'0' => (39, 0),

            1..=26 => (c0 - 1 + 4, 0x01),
            b' ' => (44, 0),
            127 => (42, 0),
            27 => (41, 0), // escape

            b'!' => (30, 0x2),
            b'@' => (31, 0x2),
            b'#' => (32, 0x2),
            b'$' => (33, 0x2),
            b'%' => (34, 0x2),
            b'^' => (35, 0x2),
            b'&' => (36, 0x2),
            b'*' => (37, 0x2),
            b'(' => (38, 0x2),
            b')' => (39, 0x2),
            b'-' => (45, 0),
            b'_' => (45, 0x2),
            b'=' => (46, 0),
            b'+' => (46, 0x2),
            b'[' => (47, 0),
            b'{' => (47, 0x2),
            b']' => (48, 0),
            b'}' => (48, 0x2),
            b'\\' => (49, 0),
            b'|' => (49, 0x2),

            /* 50 is ... */
            b';' => (51, 0),
            b':' => (51, 0x02),

            b'\'' => (52, 0),
            b'"' => (52, 0x2),

            b'`' => (53, 0),
            b'~' => (53, 0x2),

            b',' => (54, 0),
            b'<' => (54, 0x2),
            b'.' => (55, 0),
            b'>' => (55, 0x2),

            b'/' => (56, 0),
            b'?' => (56, 0x2),

            _ => (0, 0),
        };
        report[0] = m;
        report[2] = c;
        debug!("Char: {:?}, send {:X?}\r\n", &character, &report);
        // write keydown
        f.write_all(&report).unwrap();
        // write key-up
        f.write_all(&report_off).unwrap();
    }
}
