use std::{cmp, env, fs::File, io::BufWriter, io::Write};
use rand::prelude::*;

/// # Usage
///
/// generate-file-rs {size} {path}
fn main() {
    let args: Vec<String> = env::args().collect();

    let size: usize = String::from(&args[1]).parse().unwrap();
    let path = String::from(&args[2]);

    let f = File::create(path).unwrap();

    let mut buffer = [0; 1024];
    let mut writer = BufWriter::new(f);
    let mut remaining_size = size;

    let mut rng = rand::thread_rng();
    while remaining_size > 0 {
        let to_write = cmp::min(remaining_size, buffer.len());
        let buffer = &mut buffer[..to_write];
        rng.fill(buffer);
        writer.write(buffer).unwrap();

        remaining_size -= to_write;
    }
}
