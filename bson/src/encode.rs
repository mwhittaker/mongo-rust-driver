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
use std::vec::Vec;
use std::io::BufWriter;

trait Encodable<T: Clone> {
    fn encode(self) -> Vec<u8>;
}

impl Encodable<i32> for i32 {
    fn encode(self) -> Vec<u8> {
        let mut buf = [0, ..4];
        {
            let mut writer = BufWriter::new(buf);
            writer.write_le_i32(self);
        }
        Vec::from_slice(buf)
    }
}

impl Encodable<i64> for i64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = [0, ..8];
        {
            let mut writer = BufWriter::new(buf);
            writer.write_le_i64(self);
        }
        Vec::from_slice(buf)
    }
}

impl Encodable<f64> for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = [0, ..8];
        {
            let mut writer = BufWriter::new(buf);
            writer.write_le_f64(self);
        }
        Vec::from_slice(buf)
    }
}

impl Encodable<Subtype> for Subtype {
    fn encode(self) -> Vec<u8> {
        match self {
            Generic     => vec!(0x0),
            Function    => vec!(0x1),
            Binary      => vec!(0x2),
            UUID        => vec!(0x3),
            MD5         => vec!(0x4),
            UserDefined => vec!(0x80),
        }
    }        
}

pub fn encode(d: &Document) -> Vec<u8> {
    let Document(size, ref elements) = *d;
    size.encode()
        .append(elements
            .iter()
            .map(encode_element)
            .fold(Vec::new(), |a, x| a.append(x.as_slice()))
            .as_slice())
        .append_one(0)
}

fn encode_element(e: &Element) -> Vec<u8> {
    //let mut vec = Vec::new();
    let Element(ref name, ref v) = *e;
    let vbytes = match *v {
        VDouble(ref f)               => f.encode(),
        VString(ref s)               => s.clone().into_bytes().append_one(0x0),
        VDocument(ref d)             => encode(d),
        VArray(ref d)                => encode(d),
        VBinary(ref i, ref s, ref v) => i.encode()
                                         .append(s.encode().as_slice())
                                         .append(v.as_slice()),
        VObjectId(ref v)             => v.clone(),
        VFalse                       => vec!(0x0),
        VTrue                        => vec!(0x1),
        VDatetime(ref i)             => i.encode(),
        VNull                        => vec!(),
        VRegex(ref a, ref b)         => a.clone().into_bytes().append_one(0x0).append(
                                        b.clone().into_bytes().append_one(0x0).as_slice()),
        VJavascript(ref s)           => s.clone().into_bytes().append_one(0x0),
        VInt(ref i)                  => i.encode(),
        VTimestamp(ref i)            => i.encode(),
        VMinKey                      => vec!(),
        VManKey                      => vec!(),
    };
    name.clone().into_bytes().append_one(0x0).append(vbytes.as_slice())
}
