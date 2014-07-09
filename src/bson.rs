
pub struct Bson;

#[link(name = "bson-1.0")]
extern {}

impl Bson {
    pub fn hello(self) -> int {
        69
    }
}
