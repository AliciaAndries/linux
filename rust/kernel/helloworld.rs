use crate::prelude::*;
/* 
module! {
    type: RustHelloWorld,
    name: b"helloworld",
    author: b"me",
    description: b"wee test",
    license: b"dunno",
} */

struct RustHelloWorld {
    message: String,
}

impl crate::Module for RustHelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World driver included into lib.rs please just work mate\n");

        Ok(RustHelloWorld {
            message: "Hello World!".try_to_owned()?,
        }) 
    }
}

impl Drop for RustHelloWorld {
    fn drop(&mut self) {
        pr_info!("My message is {}\n", self.message);
        pr_info!("Rust Hello world is exiting\n");
    }
}