
use std::any::Void;
pub struct Bson;

type Ptr = *mut Void;

#[link(name = "bson-1.0")]
extern {
    fn bson_new() -> Ptr;
}

impl Bson {
    pub fn hello(self) -> int {
        69
    }
}

pub fn five() -> Ptr {
    unsafe { 
        bson_new()
    }
}
