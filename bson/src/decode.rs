use std::vec::Vec;
use std::io::BufReader;
use super::Document;

#[deriving(Show)]
pub struct Element(String, Value);

#[deriving(Show)]
pub enum Value {
    V_Double(f64),
    V_String(String),
    V_Document(Document),
    V_Array(Document),
    V_Binary(i32, Subtype, Vec<u8>),
//    V_ObjectId([u8, ..12]),
    V_False,
    V_True,
    V_Datetime(i64),
    V_Null,
    V_Regex(String, String),
    V_Javascript(String),
    V_Int(i32),
    V_Timestamp(i64),
    V_MinKey,
    V_ManKey
}

#[deriving(Show)]
pub enum Subtype {
    Generic,
    Function,
    Binary,
    UUID,
    MD5,
    UserDefined
}


fn parse_document(reader: &mut BufReader) -> Document {
    let n = reader.read_le_i32().unwrap();
    Document(parse_elist(reader))
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

fn parse_element(reader: &mut BufReader) -> Option<Element> {
    let t = reader.read_byte().unwrap();
    if t == 0 {
        None
    } else {
        let mut name_bytes = reader.read_until(0).unwrap();
        name_bytes.pop();
        let name = String::from_utf8(name_bytes).unwrap();
        let value = match t {
            0x01 => V_Double(reader.read_le_f64().unwrap()),
            0x02 => V_String(reader.read_to_string().unwrap()),
            0x03 => V_Document(parse_document(reader)),
            0x04 => V_Array(parse_document(reader)),
            0x10 => V_Int(reader.read_le_i32().unwrap()),
            _ => fail!("Corrupted doc!")
        };
        Some(Element(name, value))
    }
}
