#![crate_id = "ogg#0.1"]

//use std::path::posix::Path;
//use std::option::Option;
//use std::vec::Vec;

// A bitstream is represented by the stream itself (read to a certain point),
// plus any information needed to process the remainder of the stream at that
// point
pub struct OggBitstream {
    bitstream : Box<Reader>,
    end_of_stream : bool,
    current_segment_lengths : Vec<u8>
}

impl OggBitstream {
    pub fn new(b : Box<Reader>) -> OggBitstream {
        OggBitstream {
            end_of_stream : false,
            bitstream : b,
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
    pub fn next_packet(&mut self) -> Option<Vec<u8>> {
        let mut packet : Vec<u8> = Vec::new();
        loop {

            match self.next_segment() {
                None => { return None }
                Some(segment) => {
                    if segment.len() < 255 {
                        return Some(packet.append(segment.as_slice()));
                    } else {
                        packet = packet.append(segment.as_slice());
                    }
                }
            }
        }
    }

    // Removes the next segment off of the segment list and return it
    fn next_segment(&mut self) -> Option<Vec<u8>> {
        if self.current_segment_lengths.len() == 0 {
            if self.end_of_stream {
                return None;
            } else {
                self.read_page();
            }
        }

        let mut segment = Vec::from_elem(self.current_segment_lengths.as_slice()[0].to_uint().unwrap(), 0_u8);
        self.current_segment_lengths = Vec::from_slice(self.current_segment_lengths.tail());
        self.bitstream.read(segment.as_mut_slice());
        Some(segment)
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
        self.bitstream.read(header.as_mut_slice());
        let header_type = header.get(5);
        self.end_of_stream = (header_type.bitor(&0b11111011u8) == 0xffu8);
        println!("{}, {}", header_type, self.end_of_stream);
        let segment_count = header.as_slice()[26].to_uint().unwrap();
        let mut segment_table : Vec<u8> = Vec::from_elem(segment_count, 0_u8);
        self.bitstream.read(segment_table.as_mut_slice());
        self.current_segment_lengths = segment_table;
    }
}

#[cfg(test)]
mod tests {
//    extern crate ogg;

    use std::io::BufReader;
    use OggBitstream;

    #[test]
    fn some_test() {
        assert!(1 == 1);
    }

    fn make_test_packet(header_type : u8, segment : Vec<u8>) -> Vec<u8> {
        let mut packet : Vec<u8> = Vec::new();
        packet.push_all(b"OggS");
        // version
        packet.push(0);
        // header type
        packet.push(header_type);
        // absolute granule position
        packet.push_all([0, 0, 0, 0, 0, 0, 0, 0]);
        // stream serial number
        packet.push_all([0, 0, 0, 0]);
        // page sequence number
        packet.push_all([0, 0, 0, 0]);
        // checksum
        packet.push_all([0, 0, 0, 0]);
        // segment count
        packet.push(1);
        // segment table
        packet.push(segment.len().to_u8().unwrap());
        // segment itself
        packet.append(segment.as_slice())
    }

    #[test]
    fn check_short_packet() {
        // check to see that a packet with a single byte is read correctly (has
        // correct length) when we read it (and that we get a "no more packets",
        // or whatever)

        let packet = make_test_packet(7, vec![1]);
        let mut stream = OggBitstream::new(box BufReader::new(packet.as_slice()));

        match stream.next_packet() {
            None => fail!("Did not receive first packet"),
            Some(packet) => assert_eq!(1, packet.len())
        }

        match stream.next_packet() {
            None => {}
            Some(_) => fail!("got too many packets")
        }
    }
}
