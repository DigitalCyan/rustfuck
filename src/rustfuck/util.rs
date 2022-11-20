use std::io::{stdin, BufRead};

pub fn get_u8_from_console() -> u8 {
    loop {
        let mut buf = String::new();
        stdin().lock().read_line(&mut buf).unwrap();

        buf = buf.trim().to_string();

        let result = buf.parse::<u8>();

        if let Ok(n) = result {
            return n;
        }
    }
}