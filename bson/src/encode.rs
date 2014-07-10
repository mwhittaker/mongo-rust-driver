use std::ptr;
use std::io::BufReader;

//fn bson_to_doc(bson: *const bson_t) {
pub fn decode(bson: *mut bson_t) {
    unsafe {
        let ptr: *const u8 = bson_data(bson);
        let n = Int::from_le(ptr::read(ptr as *const i32)) as uint;
        let buf: &[u8] =
            std::mem::transmute(
                std::raw::Slice { data: ptr.offset(4), len: n });
        let mut reader = BufReader::new(buf);

        let result = match reader.read_byte() {
            Ok(x) => x,
            Err(_) => 0
        };
        println!("hello {}: {}", n, result);
    }
}
