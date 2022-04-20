use kernel::prelude::*;
use core::sync::atomic::{AtomicI32, Ordering};
use core::mem::size_of;
use core::str::FromStr;
//use lazy_static;
use kernel::str::CString;

const CDROM_STR_SIZE: kernel::bindings::umode_t = 1000;     // Should this be usize or umode_t

static DEBUG: bool = false;

static AUTOCLOSE: bool = true;      // TODO: make sure this static is the correct static
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

struct CDromSysctlSettings {
    info:       [kernel::c_types::c_char; CDROM_STR_SIZE as usize],
    autoclose:  bool,
    autoeject:  bool,
    debug:      bool,
    lock:       bool,
    check:      bool,   
}

/* lazy_static!{
    static ref CDROM_SYSCTL_SETTINGS = CDromSysctlSettings{
        info: [0; CDROM_STR_SIZE as usize]
        autoclose: AUTOCLOSE,
        autoeject: AUTOEJECT,
        debug: DEBUG,
        lock: LOCKDOOR,
        check: CHECK_MEDIA_TYPE,
    }
} */

static mut CDROM_SYSCTL_SETTINGS: CDromSysctlSettings = CDromSysctlSettings{
    info: [0; CDROM_STR_SIZE as usize],
    autoclose: AUTOCLOSE,
    autoeject: AUTOEJECT,
    debug: DEBUG,
    lock: LOCKDOOR,
    check: CHECK_MEDIA_TYPE
};

static cdrom_sysctl_header: *const kernel::bindings::ctl_table_header;  // TODO: make this a lazy static

struct RustCDRom {                                  // TODO: this should maybe be another struct: cdrom_sysctl_header
    text: String,
}

//static cdrom_sysctl_header: *const ctl_table_header;        // AL: this shouldnt work and it doesn't isnt that fun

impl kernel::Module for RustCDRom {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("init rust cdrom\n");

        let cdrom_table =
        [
        kernel::bindings::ctl_table {
            procname:       unsafe{CString::new("info").unwrap().as_ptr as *const u8},
            data:           CDROM_SYSCTL_SETTINGS.info,        // refers to a struct that should be the same struct as the one used in the c-code
            maxlen:         CDROM_STR_SIZE,
            mode:           0444,
            proc_handler:   Some(kernel::bindings::cdrom_sysctl_info)   // function pointer

        },
        kernel::bindings::ctl_table {
            procname:	    i8::from_str("autoclose"),
            data:		    CDROM_SYSCTL_SETTINGS.autoclose,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	Some(kernel::bindings::cdrom_sysctl_handler),
        },
        kernel::bindings::ctl_table {
            procname:       i8::from_str("autoeject"),
            data:		    CDROM_SYSCTL_SETTINGS.autoeject,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	Some(kernel::bindings::cdrom_sysctl_handler),
        },
        kernel::bindings::ctl_table {
            procname:	    i8::from_str("debug"),
            data:		    CDROM_SYSCTL_SETTINGS.debug,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:   Some(kernel::bindings::cdrom_sysctl_handler),
        },
        kernel::bindings::ctl_table {
            procname:	    i8::from_str("lock"),
            data:		    CDROM_SYSCTL_SETTINGS.lock,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	Some(kernel::bindings::cdrom_sysctl_handler),
        },
        kernel::bindings::ctl_table {
            procname:	    CString::new("check_media").unwrap.as_ptr as *const u8,
            data:		    CDROM_SYSCTL_SETTINGS.check,
            maxlen:		    size_of::<i32>(),
            mode:		    0644,
            proc_handler:	Some(kernel::bindings::cdrom_sysctl_handler)
        },
        ];
    
        static INITIALIZED : AtomicI32  = AtomicI32::new(0);
    
        
        if !atomic_add_unless(&INITIALIZED, 1, 1){
            return Ok(RustCDRom{text: "already existed"}); //this shouldnt really be updated by an unwanted initalization
        }
        
        //what happens if I mark this line unsafe -> its out of scope later so completely useless (this was for when it was still in RustCDRom)
        //let sysctl_header = kernel::bindings::register_sysctl(i8::from_str("dev/cdrom"), &mut cdrom_table);
        //cdrom_table should be a pointer to the first element in the array, so that also aint right
        unsafe{
            cdrom_sysctl_header = kernel::bindings::register_sysctl(i8::from_str("dev/cdrom"), &mut cdrom_table);
        }
        

        CDROM_SYSCTL_SETTINGS.autoclose = AUTOCLOSE;
        CDROM_SYSCTL_SETTINGS.autoeject = AUTOEJECT;
        CDROM_SYSCTL_SETTINGS.debug = DEBUG;
        CDROM_SYSCTL_SETTINGS.lock = LOCKDOOR;
        CDROM_SYSCTL_SETTINGS.check = CHECK_MEDIA_TYPE;

        Ok(RustCDRom{
            text: "initialized"
        })
    }
}

impl Drop for RustCDRom {
    fn drop(&mut self) {
        kernel::bindings::unregister_sysctl_table(self.cdrom_sysctl_header);
        pr_info!("exiting cdrom");
    }
}