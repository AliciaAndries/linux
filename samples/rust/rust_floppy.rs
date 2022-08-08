use kernel::{prelude::*, chrdev};
use kernel::{str, c_str, file};


module! {
    type: Floppy,
    name: b"floppy",
    license: b"GPM",
}


struct RustFile;

impl file::Operations for RustFile {
    kernel::declare_file_operations!();

    fn open(_shared: &(), _file: &file::File) -> Result {
        Ok(())
    }
}

struct Floppy {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl kernel::Module for Floppy {
    fn init(name: &'static str::CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("floppy disk module has started");
        let mut reg = chrdev::Registration::new_pinned(name, 0, module)?;
        reg.as_mut().register::<RustFile>()?;
        Ok(Floppy {
            _dev: reg,
        })
    }
}

