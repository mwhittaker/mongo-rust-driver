use std::vec::Vec;
use std::io::BufReader;
use std::ptr;
use std::raw;
use std::mem;

use super::bson_t;
use super::bson_get_data;
use super::Document;
use super::Element;
use super::Value;
    use super::VDouble;
    use super::VString;
    use super::VDocument;
    use super::VArray;
    use super::VBinary;
    use super::VObjectId;
    use super::VFalse;
    use super::VTrue;
    use super::VDatetime;
    use super::VNull;
    use super::VRegex;
    use super::VJavascript;
    use super::VInt;
    use super::VTimestamp;
    use super::VMinKey;
    use super::VManKey;
use super::Generic;
use super::Function;
use super::Binary;
use super::UUID;
use super::MD5;
use super::UserDefined;
use super::Subtype;

pub fn parse_document(reader: &mut BufReader) -> Document {
    let n = reader.read_le_i32().unwrap();
    Document(n, parse_elist(reader))
}

fn parse_elist(reader: &mut BufReader) -> Vec<Element> {
    match parse_element(reader) {
        Some(x) => {
            let mut v = parse_elist(reader);
            v.unshift(x);
            v
        }
        None => Vec::new()
    }
}

fn read_cstring(reader: &mut BufReader) -> String {
    let mut bytes = reader.read_until(0).unwrap();
    bytes.pop();
    String::from_utf8(bytes).unwrap()
}

fn parse_element(reader: &mut BufReader) -> Option<Element> {
    let t = reader.read_byte().unwrap();
    if t == 0 {
        None
    } else {
        let name = read_cstring(reader);
        let value = match t {
            0x01 => VDouble(reader.read_le_f64().unwrap()),
            0x02 => VString(read_cstring(reader)),
            0x03 => VDocument(parse_document(reader)),
            0x04 => VArray(parse_document(reader)),
            0x10 => VInt(reader.read_le_i32().unwrap()),
            _ => fail!("Corrupted doc! t={}, name={}", t, name)
        };
        Some(Element(name, value))
    }
}
