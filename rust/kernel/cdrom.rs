use kernel::prelude::*;
use kernel::{
    bindings};
use core::sync::atomic::{AtomicI32, Ordering};
use crate::c_types;
use core::mem;

const CDROM_STR_SIZE: bindings::umode_t = 1000

/* pub struct cdrom_sysctl_settings {                  // bindgen the og and dont use this
    info:       [c_types::c_char; CDROM_STR_SIZE]
    autoclose:  i32,
    autoeject:  i32,
    debug:      i32,
    lock:       i32,
    check:      i32   
} */

/* module! {
    type: RustCDRom,
    name: b"rust_cdrom",
    author: b"Me",
    description: b"Rust version of cdrom driver code",
    license: b"GPL v2/probs best check this out",
} */

fn atomic_add_unless(atom: &AtomicI32, add: i32, comp: i32)->bool{
    let mut a = atom.load(Ordering::Acquire);
    if a == comp {
        return false;
    }
    a = a + add;
    atom.store(a, Ordering::Release);
    return true;
}

/* fn cdrom_print_info()->i32{

    let mut ret = bindings::scnprintf(info + pos, CDROM_STR_SIZE - pos, header);

    if !ret {
        return 1;
    }

    for 
} */

/*  TODO
 *  table only used once and in the register code so can probs pass struct from C to here
 *  need to make sure that there is some kind of mutex or something I think, 
 *  the struct cdrom_sysctl_settings is static but not initialized so confusing
 */

/* let cdrom_table: [bindings::ctl_table; 6] =
[
{
    procname:       "info",
    data:           &cdrom_sysctl_settings.info,        // refers to a struct that should be the same struct as the one used in the c-code
    maxlen:         CDROM_STR_SIZE,
    mode:           0444,
    proc_handler:   cdrom_sysctl_info   // function pointer
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
} */

fn cdrom_sysctl_register_rust(cdrom_sysctl_settings: *bindings::cdrom_sysctl_settings,
        autoclose: bool,  autoeject: bool, debug: bool, 
        lockdoor: bool, check_media_type: bool) -> i32 {
    pr_info!("rust register of cdrom driver")

    let cdrom_table: [bindings::ctl_table; 7] =
    [
    {
        procname:       "info",
        data:           cdrom_sysctl_settings.info,        // refers to a struct that should be the same struct as the one used in the c-code
        maxlen:         CDROM_STR_SIZE,
        mode:           0444,
        proc_handler:   bindings::cdrom_sysctl_info   // function pointer
    },
    {
		procname:	    "autoclose",
		data:		    cdrom_sysctl_settings.autoclose,
		maxlen:		    size_of<i32>(),
		mode:		    0644,
		proc_handler:	bindings::cdrom_sysctl_handler,
	},
    {
		procname:       "autoeject",
		data:		    &cdrom_sysctl_settings.autoeject,
		maxlen:		    size_of<i32>(),
		mode:		    0644,
		proc_handler:	bindings::cdrom_sysctl_handler,
	},
	{
		procname:	    "debug",
		data:		    cdrom_sysctl_settings.debug,
		maxlen:		    size_of<i32>(),
		mode:		    0644,
		proc_handler:   bindings::cdrom_sysctl_handler,
	},
	{
		procname:	    "lock",
		data:		    cdrom_sysctl_settings.lock,
		maxlen:		    size_of<i32>(),
		mode:		    0644,
		proc_handler:	bindings::cdrom_sysctl_handler,
	},
	{
		procname:	    "check_media",
		data:		    cdrom_sysctl_settings.check,
		maxlen:		    size_of<i32>(),
		mode:		    0644,
		proc_handler:	bindings::cdrom_sysctl_handler
	},
	{ }
    ];

    static INITIALIZED : AtomicI32  = AtomicI32::new(0);

    
    if !atomic_add_unless(&INITIALIZED, 1, 1){
        return;
    }

    kernel::bindings::register_sysctl("dev/cdrom", cdrom_table)

    cdrom_sysctl_settings.autoclose = autoclose;
    cdrom_sysctl_settings.autoeject = autoeject;
    cdrom_sysctl_settings.debug = debug;
    cdrom_sysctl_settings.lock = lockdoor;
    cdrom_sysctl_settings.check = check_media_type;
}
