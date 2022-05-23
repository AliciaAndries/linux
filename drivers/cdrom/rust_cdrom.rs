use kernel::prelude::*;
use core::sync::atomic::{AtomicI32, Ordering};
use core::mem::size_of;
use core::str::FromStr;
//use lazy_static;
use kernel::str::CString;
use kernel::sysctl::Sysctl;
use alloc::fmt::Arguments;
use kernel::c_types;
use kernel::bindings;
use core::sync::atomic;
use core::format_args;


const CDROM_STR_SIZE: kernel::bindings::umode_t = 1000;     // Should this be usize or umode_t

static DEBUG: atomic::AtomicBool = atomic::AtomicBool::new(false);

static AUTOCLOSE: atomic::AtomicBool = atomic::AtomicBool::new(true);      // TODO: make sure this static is the correct static
static AUTOEJECT: atomic::AtomicBool = atomic::AtomicBool::new(false);
static LOCKDOOR: atomic::AtomicBool = atomic::AtomicBool::new(true);

static CHECK_MEDIA_TYPE: atomic::AtomicBool = atomic::AtomicBool::new(false);

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
    info:       [c_types::c_char; CDROM_STR_SIZE as usize],
    autoclose:  atomic::AtomicBool,
    autoeject:  atomic::AtomicBool,
    debug:      atomic::AtomicBool,
    lock:       atomic::AtomicBool,
    check:      atomic::AtomicBool,   
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

//static cdrom_sysctl_header: *const kernel::bindings::ctl_table_header;  // TODO: make this a lazy static

struct RustCDRom {                                  // TODO: this should maybe be another struct: cdrom_sysctl_header
    cdrom_sysctl_info: Result<Sysctl<atomic::AtomicBool>>,
    cdrom_sysctl_autoclose: Result<Sysctl<atomic::AtomicBool>>,
    cdrom_sysctl_autoeject: Result<Sysctl<atomic::AtomicBool>>,
    cdrom_sysctl_debug: Result<Sysctl<atomic::AtomicBool>>,
    cdrom_sysctl_lock: Result<Sysctl<atomic::AtomicBool>>,
    cdrom_sysctl_check_media: Result<Sysctl<atomic::AtomicBool>>,
    //cdrom_sysctl_settings: CDromSysctlSettings,
}

//static cdrom_sysctl_header: *const ctl_table_header;        // AL: this shouldnt work and it doesn't isnt that fun

impl kernel::Module for RustCDRom {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("init rust cdrom\n");

/*         let cdrom_table =
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
        ]; */
    
        static INITIALIZED : AtomicI32  = AtomicI32::new(0);
    
        
        if !atomic_add_unless(&INITIALIZED, 1, 1){
            return Err(EINVAL); //this shouldnt really be updated by an unwanted initalization
        }
        
        //what happens if I mark this line unsafe -> its out of scope later so completely useless (this was for when it was still in RustCDRom)
        //let sysctl_header = kernel::bindings::register_sysctl(i8::from_str("dev/cdrom"), &mut cdrom_table);
        //cdrom_table should be a pointer to the first element in the array, so that also aint right
        /* unsafe{
            cdrom_sysctl_header = kernel::bindings::register_sysctl(i8::from_str("dev/cdrom"), &mut cdrom_table);
        } */
        

        CDROM_SYSCTL_SETTINGS.autoclose = AUTOCLOSE;
        CDROM_SYSCTL_SETTINGS.autoeject = AUTOEJECT;
        CDROM_SYSCTL_SETTINGS.debug = DEBUG;
        CDROM_SYSCTL_SETTINGS.lock = LOCKDOOR;
        CDROM_SYSCTL_SETTINGS.check = CHECK_MEDIA_TYPE;

        let path = CString::try_from_fmt(format_args!("dev/cdrom")).unwrap();

        Ok(RustCDRom{
            cdrom_sysctl_autoclose: Sysctl::<atomic::AtomicBool>::custom_register(&path, &CString::try_from_fmt(format_args!("autoclose")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.autoclose, 0644 as bindings::umode_t, kernel::bindings::cdrom_sysctl_handler, size_of::<i32>()),
            cdrom_sysctl_autoeject: Sysctl::<atomic::AtomicBool>::custom_register(&path, &CString::try_from_fmt(format_args!("autoeject")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.autoeject, 0644 as bindings::umode_t, kernel::bindings::cdrom_sysctl_handler, size_of::<i32>()),
            cdrom_sysctl_debug: Sysctl::<atomic::AtomicBool>::custom_register(&path, &CString::try_from_fmt(format_args!("debug")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.debug, 0644 as bindings::umode_t, kernel::bindings::cdrom_sysctl_handler, size_of::<i32>()),
            cdrom_sysctl_check_media: Sysctl::<atomic::AtomicBool>::custom_register(&path, &CString::try_from_fmt(format_args!("check_media")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.check, 0644 as bindings::umode_t, kernel::bindings::cdrom_sysctl_handler, size_of::<i32>()),
            cdrom_sysctl_lock: Sysctl::<atomic::AtomicBool>::custom_register(&path, &CString::try_from_fmt(format_args!("lock")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.lock, 0644 as bindings::umode_t, kernel::bindings::cdrom_sysctl_handler, size_of::<i32>()),
            cdrom_sysctl_info: Sysctl::<&[i8; 1000]>::custom_register(&path, &CString::try_from_fmt(format_args!("info")).unwrap(), 
                CDROM_SYSCTL_SETTINGS.info, 0444 as bindings::umode_t, kernel::bindings::cdrom_sysctl_info, CDROM_STR_SIZE as usize),
        })
    }
}

impl Drop for RustCDRom {
    fn drop(&mut self) {
        drop(self.cdrom_sysctl_autoclose.unwrap());
        pr_info!("exiting cdrom");
    }
}