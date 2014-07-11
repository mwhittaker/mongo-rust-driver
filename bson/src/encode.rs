use super::Document;
use super::Element;
use super::Value;
use super::Subtype;
use std::vec::Vec;

pub fn encode(d: Document) -> Vec<u8> {
    let Document(size, elements) = d;
    elements
        .iter()
        .map(encode_element)
        .fold(Vec::new(), |a, x| a.append(x.as_slice()))
}

fn encode_element(v: &Element) -> Vec<u8> {
    let mut v = Vec::new();
    v.push(1u8);
    v
}
