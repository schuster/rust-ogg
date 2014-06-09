use std::io::File;
use std::path::posix::Path;
//use std::option::Option;
use std::vec::Vec;

// Prints the byte length of every underlying packet in the stream

// A bitstream is represented by the stream itself (read to a certain point),
// plus any information needed to process the remainder of the stream at that
// point
struct OggBitstream {
    file : File,
    end_of_stream : bool,
    current_segment_lengths : Vec<u8>
}

impl OggBitstream {
    fn new(f : File) -> OggBitstream {
        OggBitstream {
            end_of_stream : false,
            file : f,
            current_segment_lengths : Vec::new() }
    }

    /*
packet header:
OggS: 4 bytes
version: 1 byte
header type: 1 byte (fresh/continued packet, bos, eos)
absolute granule position: 8 bytes
stream serial number: 4 bytes
page sequence number: 4 bytes
page checksum: 4 bytes
segment count: 1 byte
segment table (variable: 1 byte per segment)
*/

    // Returns the bytes of the next packet in the underlying stream, or a
    // suitable null value
    fn next_packet(&mut self) -> Option<Vec<u8>> {
        let mut packet : Vec<u8> = Vec::new();
        loop {
            let segment = self.next_segment();
            if segment.len() < 255 {
                return Some(packet.append(segment.as_slice()));
            } else {
                packet = packet.append(segment.as_slice());
            }
        }
    }

    // Removes the next segment off of the segment list and return it
    fn next_segment(&mut self) -> Vec<u8> {
        if self.current_segment_lengths.len() == 0 {
            self.read_page();
        }

        // TODO: check end of stream

        let mut segment = Vec::from_elem(self.current_segment_lengths.as_slice()[0].to_uint().unwrap(), 0_u8);
        self.current_segment_lengths = Vec::from_slice(self.current_segment_lengths.tail());
        self.file.read(segment.as_mut_slice());
        segment
    }

    // reads the next page of the bitstream (through the segment table, but not
    // the segments themselve), writing it to the struct
    fn read_page(&mut self) {
        println!("Reading a page")
        // 1. read the static-length body
        // 2. read the variable-length current-segments table, write it to struct

        // TODO: figure out how to create the vector to read into without
        // initializing it first (maybe use a BufferedReader)
        let mut header = Vec::from_elem(27, 0_u8);
        self.file.read(header.as_mut_slice());
        let segment_count = header.as_slice()[26].to_uint().unwrap();
        let mut segment_table : Vec<u8> = Vec::from_elem(segment_count, 0_u8);
        self.file.read(segment_table.as_mut_slice());
        self.current_segment_lengths = segment_table;
    }
}

fn main() {
    println!("starting...");
    let mut maybe_f = File::open(&Path::new("star_wars.ogg"));
    match maybe_f {
        Ok(mut f) => {

            //let mut header : Vec<u8> = Vec::from_elem(27, 0_u8);
            // println!("vector reference length: {}", header.len());
            // println!("vector reference capacity: {}", header.capacity());
            // let mut header_slice = header.mut_slice(0, 27);
            // println!("slice length: {}", header_slice.len());
            // match f.read_at_least(3, header_slice) {
            //     Ok(bytes_read) => { println!("bytes read: {}", bytes_read); }
            //     Err(error) => { println!("error {}", error); }
            // }

            //     //println!("{}", header_slice.len());
            let mut bs = OggBitstream::new(f);
            let mut next_packet = bs.next_packet();
            while next_packet.is_some() {
                println!("Packet: {}", next_packet.unwrap().len());
                next_packet = bs.next_packet();
            }
        }
        _ => ()
    }
}
