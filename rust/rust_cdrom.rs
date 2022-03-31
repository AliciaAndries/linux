use kernel::prelude::*;
use kernel::{
    bindings};
use core::sync::atomic::{AtomicI32, Ordering};
use crate::c_types;

const CDROM_STR_SIZE: bindings::umode_t = 1000

pub struct cdrom_sysctl_settings {
    info:       [char; CDROM_STR_SIZE]
    autoclose:  i32,
    autoeject:  i32,
    debug:      i32,
    lock:       i32,
    check:      i32   
}

module! {
    type: RustCDRom,
    name: b"rust_cdrom",
    author: b"Me",
    description: b"Rust version of cdrom driver code",
    license: b"GPL v2/probs best check this out",
}

fn atomic_add_unless(atom: &AtomicI32, add: i32, comp: i32)->bool{
    let mut a = atom.load(Ordering::Acquire);
    if a == comp {
        return false;
    }
    a = a + add;
    atom.store(a, Ordering::Release);
    return true;
}

let cdrom_table: [bindings::ctl_table; 6] =
[
{
    procname:       "info",
    data:           &cdrom_sysctl_settings.info,
    maxlen:         CDROM_STR_SIZE,
    mode:           0444,
    proc_handler:   cdrom_sysctl_info   //function pointer
}
];

impl KernelModule for RustCDRom {
    fn init name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust init of cdrom driver")
        
        static INITIALIZED : AtomicI32  = AtomicI32::new(0);

    
        if !atomic_add_unless(&INITIALIZED, 1, 1){
            return;
        }

        bindings::register_sysctl("dev/cdrom", cdrom_table)
    }
}
