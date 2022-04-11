use kernel::prelude::*;
use core::sync::atomic::{AtomicI32, Ordering};
use core::mem::size_of;

const CDROM_STR_SIZE: kernel::bindings::usize = 1000;     // Should this be usize or umode_t

static DEBUG: bool = false;

static AUTOCLOSE: bool = true;
static AUTOEJECT: bool = false;
static LOCKDOOR: bool = true;

static CHECK_MEDIA_TYPE: bool = false;

module! {
    type: RustCDRom,
    name: b"rust_cdrom",
    author: b"me",
    description: b"wee test",
    license: b"dunno",
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

struct RustCDRom {                                  // TODO: this should maybe be another struct: cdrom_sysctl_header
    info:       [kernel::c_types::c_char; CDROM_STR_SIZE],
    autoclose:  bool,
    autoeject:  bool,
    debug:      bool,
    lock:       bool,
    check:      bool   
}

static cdrom_sysctl_header: *const ctl_table_header;        // AL: this shouldnt work and it doesn't isnt that fun

impl kernel::Module for RustCDRom {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("init rust cdrom\n");

        let cdrom_table =
        [
        kernel::bindings::ctl_table {
            procname:       "info",
            data:           RustCDRom::info,        // refers to a struct that should be the same struct as the one used in the c-code
            maxlen:         CDROM_STR_SIZE,
            mode:           0444,
            proc_handler:   kernel::bindings::cdrom_sysctl_info   // function pointer
        },
        kernel::bindings::ctl_table {
            procname:	    "autoclose",
            data:		    RustCDRom::autoclose,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	kernel::bindings::cdrom_sysctl_handler,
        },
        kernel::bindings::ctl_table {
            procname:       "autoeject",
            data:		    RustCDRom::autoeject,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	kernel::bindings::cdrom_sysctl_handler,
        },
        kernel::bindings::ctl_table {
            procname:	    "debug",
            data:		    RustCDRom::debug,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:   kernel::bindings::cdrom_sysctl_handler,
        },
        kernel::bindings::ctl_table {
            procname:	    "lock",
            data:		    RustCDRom::lock,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	kernel::bindings::cdrom_sysctl_handler,
        },
        kernel::bindings::ctl_table {
            procname:	    "check_media",
            data:		    RustCDRom::check,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	kernel::bindings::cdrom_sysctl_handler
        },
        ];
    
        static INITIALIZED : AtomicI32  = AtomicI32::new(0);
    
        
        if !atomic_add_unless(&INITIALIZED, 1, 1){
            return RustCDRom;                            // AL: should this be self or none?
        }
    
        cdrom_sysctl_header = kernel::bindings::register_sysctl("dev/cdrom", cdrom_table);     // am i sending a copy of the array?

        Ok(RustCDRom {                  // AL: I think this struct shouldnt be in Ok as info is not set, rather, it should be the last line but how do you return then?
            autoclose: AUTOCLOSE,
            autoeject: AUTOEJECT,
            debug: DEBUG,
            lock: LOCKDOOR,
            check: CHECK_MEDIA_TYPE,
        }) 
    }
}

impl Drop for RustCDRom {
    fn drop(&mut self) {
        kernel::bindings::unregister_sysctl_table(cdrom_sysctl_header);
        pr_info!("exiting cdrom");
    }
}