use std::ptr;
use std::mem;
use std::io::BufReader;
use std::slice::raw::buf_as_slice;
use std::raw::Slice;

use super::bson_t;
use super::bson_get_data;

//fn bson_to_doc(bson: *const bson_t) {
pub fn decode(bson: *const super::bson_t) {
    unsafe {
        let ptr: *const u8 = super::bson_get_data(bson);
        let n = Int::from_le(ptr::read(ptr as *const i32)) as uint;
        let buf: &[u8] =
            mem::transmute(Slice { data: ptr, len: n });
        let mut reader = BufReader::new(buf);
        parse_document(&reader);
    }
}

fn parse_document(reader: &BufReader) -> Document {
    let n = reader.read_le_i32().unwrap();
    let elements = parse_elist(reader);
}

fn parse_elist(reader: &BufReader) -> Vec<Element> {
    match parse_element(reader) {
        Some(x) => parse_elist(reader).unshift(x),
        None => Vec<Element>::new()
    }
}

fn parse_element(reader: &BufReader) -> Option<Element> {
    match reader.read_byte() {
        Ok(0x00) => None,
        Ok(0x01) => 1,
        Ok(0x02) => 2,
        Ok(0x03) => 3,
        _ => fail!("Corrupted doc!"),
    }
}
