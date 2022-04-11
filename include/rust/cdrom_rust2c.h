#include <rust/cdrom_c2rust.h>


int cdrom_sysctl_register_rust(struct cdrom_sysctl_settings* cdrom_sysctl_settings,
        bool autoclose, bool autoeject, bool debug, 
        bool lockdoor, bool check_media_type);

