extern crate libc;

pub enum bson_t {}
pub enum bson_error_t {}
pub enum bson_realloc_func {}
pub enum bson_validate_flags_t {}

#[link(name = "bson-1.0")]
extern {
    fn bson_new() -> *mut bson_t;
    fn bson_new_from_json(data: *const libc::uint8_t, len: libc::size_t, error: *mut bson_error_t) -> *mut bson_t;
    fn bson_init_from_json(bson: *mut bson_t, data: *const libc::c_char, len: libc::size_t, error: *mut bson_error_t) -> bool; 
    fn bson_init_static(b: *mut bson_t, data: *const libc::uint8_t, length: libc::size_t) -> bool; 
    fn bson_init(b: *mut bson_t) -> libc::c_void; 
    fn bson_reinit(b: *mut bson_t) -> libc::c_void; 
    fn bson_new_from_data(data: *const libc::uint8_t, length: libc::size_t) -> bson_t; 
    fn bson_new_from_buffer(buf: *mut *mut libc::uint8_t, buf_len: *mut libc::size_t, realloc_func: bson_realloc_func, realloc_func_ctx: *mut libc::c_void) -> bson_t; 
    fn bson_sized_new(size: libc::size_t) -> bson_t; 
    fn bson_copy(bson: *const bson_t) -> bson_t; 
    fn bson_copy_to(src: *const bson_t, std: *mut bson_t) -> libc::c_void; 
    fn bson_destroy(bson: *mut bson_t) -> libc::c_void; 
    fn bson_destroy_with_steal(bson: *mut bson_t, steal: bool, length: *mut libc::uint32_t) -> libc::uint8_t; 
    fn bson_get_data(bson: *const bson_t) -> *const libc::uint8_t; 
    fn bson_count_keys(bson: *const bson_t) -> libc::uint32_t; 
    fn bson_has_field(bson: *const bson_t, key: *const libc::c_char) -> bool; 
    fn bson_compare(bson: *const bson_t, other: *const bson_t) -> int; 
    fn bson_equal(bson: *const bson_t, other: *const bson_t) -> bool; 
    fn bson_validate(bson: *const bson_t,flags: bson_validate_flags_t, offset: *mut libc::size_t) -> bool; 
    fn bson_as_json(bson: *const bson_t, length: *mut libc::size_t)  -> *mut libc::c_char; 
    fn bson_array_as_json(bson: *const bson_t, lenght: *mut libc::size_t) -> *mut libc::c_char; 
    
    /*
    fn bson_append_value(bson: *mut bson_t, key: *const char, key_length: int, value: *const bson_value_t) -> bool; 
    fn bson_append_array(bson: *mut bson_t, key: *const char, key_length: int, array: *const bson_t)  -> bool; 
    fn bson_append_binary(bson: *mut bson_t, key: *const char, key_length: int, bson_subtype_t subtype, const uint8_t *binary, uint32_t length) -> bool; 
    fn bson_append_bool(bson_t *bson, const char *key, int key_length, bool value)  -> bool; 
    fn bson_append_code(bson_t *bson, const char *key, int key_length, const char *javascript) -> bool; 
    fn bson_append_code_with_scope(bson_t *bson, const char *key, int key_length, const char *javascript, const bson_t *scope)  -> bool; 
    fn bson_append_dbpointer(bson_t *bson, const char *key, int key_length, const char *collection, const bson_oid_t *oid)  -> bool; 
    fn bson_append_double(bson_t *bson, const char *key, int key_length, double value) -> bool; 
    fn bson_append_document(bson_t *bson, const char *key, int key_length, const bson_t *value) -> bool; 
    fn bson_append_document_begin(bson_t *bson, const char *key, int key_length, bson_t *child) -> bool; 
    fn bson_append_document_end(bson_t *bson, bson_t *child)  -> bool; 
    fn bson_append_array_begin(bson_t *bson, const char *key, int key_length, bson_t *child) -> bool; 
    fn bson_append_array_end(bson_t *bson, bson_t *child) -> bool; 
    fn bson_append_int32(bson_t *bson, const char *key, int key_length, int32_t value) -> bool; 
    fn bson_append_int64(bson_t *bson, const char *key, int key_length, int64_t value) -> bool; 
    fn bson_append_iter(bson_t *bson, const char *key, int key_length, const bson_iter_t *iter) -> bool; 
    fn bson_append_minkey(bson_t *bson, const char *key, int key_length) -> bool; 
    fn bson_append_maxkey(bson_t *bson, const char *key, int key_length)  -> bool; 
    fn bson_append_null(bson_t *bson, const char *key, int key_length) -> bool; 
    fn bson_append_oid(bson_t *bson, const char *key, int key_length, const bson_oid_t *oid)  -> bool; 
    fn bson_append_regex(bson_t *bson, const char *key, int key_length, const char *regex, const char *options) -> bool; 
    fn bson_append_utf8(bson_t *bson, const char *key, int key_length, const char *value, int length) -> bool; 
    fn bson_append_symbol(bson_t *bson, const char *key, int key_length, const char *value, int length) -> bool; 
    fn bson_append_time_t(bson_t *bson, const char *key, int key_length, time_t value) -> bool; 
    fn bson_append_timeval(bson_t *bson, const char *key, int key_length, struct timeval *value) -> bool; 
    fn bson_append_date_time(bson_t *bson, const char *key, int key_length, int64_t value) -> bool; 
    fn bson_append_now_utc(bson_t *bson, const char *key, int key_length) -> bool; 
    fn bson_append_timestamp(bson_t *bson, const char *key, int key_length, uint32_t timestamp, uint32_t increment) -> bool; 
    fn bson_append_undefined(bson_t *bson, const char *key, int key_length) -> bool; 
    fn bson_concat (bson_t *dst, const bson_t *src) -> bool; 
    */
}


#[test]
fn foo() {
    unsafe {    
        let b = bson_new();
        let f = "{\"a\": {\"a\": 2}}".to_c_str();
        let b = bson_new_from_json(f.as_ptr() as *const u8, f.len() as u64, 0 as *mut bson_error_t);
        std::io::stdout().write_line(format!("{}", b).as_slice());
    }
}
