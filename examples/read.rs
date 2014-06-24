extern crate ogg;

use std::io::File;
use ogg::OggBitstream;

fn main() {
    println!("starting...");
    let mut maybe_f = File::open(&Path::new("star_wars.ogg"));
    match maybe_f {
        Ok(f) => {
            let mut bs = OggBitstream::new(box f);
            let mut next_packet = bs.next_packet();
            while next_packet.is_some() {
                println!("Packet: {}", next_packet.unwrap().len());
                next_packet = bs.next_packet();
            }
        }
        _ => ()
    }
}
