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
            mem::transmute(Slice { data: ptr.offset(4), len: n });
        let mut reader = BufReader::new(buf);

        let result = match reader.read_byte() {
            Ok(x) => x,
            Err(_) => 0
        };
        println!("hello {}: {}", n, result);
    }
}
