use std::ptr;
use std::mem;
use std::vec::Vec;
use std::io::BufReader;
use std::slice::raw::buf_as_slice;
use std::raw::Slice;

use super::bson_t;
use super::bson_get_data;

pub struct Document(Vec<Element>);
pub struct Element(String, Value);

pub enum Value {
    V_Double(i64),
    V_String(String),
    V_Document(Document),
    V_Array(Document),
    V_Binary(i32, Subtype, Vec<u8>),
    V_ObjectId([u8, ..12]),
    V_False,
    V_True,
    V_Datetime(i64),
    V_Null,
    V_Regex(String, String),
    V_Javascript(String),
    V_Timestamp(i64),
    V_MinKey,
    V_ManKey
}

pub enum Subtype {
    Generic,
    Function,
    Binary,
    UUID,
    MD5,
    UserDefined
}

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
    reader.read_le_i32().unwrap();
    parse_elist(reader)
}

fn parse_elist(reader: &BufReader) -> Document {
    match parse_element(reader) {
        Some(x) => { 
            let v = parse_elist(reader);
            v.unshift(x);
            v
        }
        None => Vec::new()
    }
}

fn parse_element(reader: &BufReader) -> Option<Element> {
    let t = reader.read_byte().unwrap();
    if t == 0 {
        None
    } else {
        let name = reader.read_to_string().unwrap();
        let value = match t {
            0x01 => V_Double(reader.read_le_f64().unwrap()),
            0x02 => V_String(reader.read_to_string().unwrap()),
            0x03 => V_Document(parse_document(reader)),
            0x04 => V_Array(parse_document(reader)),
            _ => fail!("Corrupted doc!")
        }
        Element(name, value)
    }
}
