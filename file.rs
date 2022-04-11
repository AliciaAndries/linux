pub struct cdrom_msf0 {
fn bindgen_test_layout_cdrom_msf0() {
        ::core::mem::size_of::<cdrom_msf0>(),
        concat!("Size of: ", stringify!(cdrom_msf0))
        ::core::mem::align_of::<cdrom_msf0>(),
        concat!("Alignment of ", stringify!(cdrom_msf0))
        unsafe { &(*(::core::ptr::null::<cdrom_msf0>())).minute as *const _ as usize },
            stringify!(cdrom_msf0),
        unsafe { &(*(::core::ptr::null::<cdrom_msf0>())).second as *const _ as usize },
            stringify!(cdrom_msf0),
        unsafe { &(*(::core::ptr::null::<cdrom_msf0>())).frame as *const _ as usize },
            stringify!(cdrom_msf0),
pub union cdrom_addr {
    pub msf: cdrom_msf0,
fn bindgen_test_layout_cdrom_addr() {
        ::core::mem::size_of::<cdrom_addr>(),
        concat!("Size of: ", stringify!(cdrom_addr))
        ::core::mem::align_of::<cdrom_addr>(),
        concat!("Alignment of ", stringify!(cdrom_addr))
        unsafe { &(*(::core::ptr::null::<cdrom_addr>())).msf as *const _ as usize },
            stringify!(cdrom_addr),
        unsafe { &(*(::core::ptr::null::<cdrom_addr>())).lba as *const _ as usize },
            stringify!(cdrom_addr),
impl Default for cdrom_addr {
pub struct cdrom_msf {
fn bindgen_test_layout_cdrom_msf() {
        ::core::mem::size_of::<cdrom_msf>(),
        concat!("Size of: ", stringify!(cdrom_msf))
        ::core::mem::align_of::<cdrom_msf>(),
        concat!("Alignment of ", stringify!(cdrom_msf))
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_min0 as *const _ as usize },
            stringify!(cdrom_msf),
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_sec0 as *const _ as usize },
            stringify!(cdrom_msf),
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_frame0 as *const _ as usize },
            stringify!(cdrom_msf),
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_min1 as *const _ as usize },
            stringify!(cdrom_msf),
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_sec1 as *const _ as usize },
            stringify!(cdrom_msf),
        unsafe { &(*(::core::ptr::null::<cdrom_msf>())).cdmsf_frame1 as *const _ as usize },
            stringify!(cdrom_msf),
pub struct cdrom_ti {
fn bindgen_test_layout_cdrom_ti() {
        ::core::mem::size_of::<cdrom_ti>(),
        concat!("Size of: ", stringify!(cdrom_ti))
        ::core::mem::align_of::<cdrom_ti>(),
        concat!("Alignment of ", stringify!(cdrom_ti))
        unsafe { &(*(::core::ptr::null::<cdrom_ti>())).cdti_trk0 as *const _ as usize },
            stringify!(cdrom_ti),
        unsafe { &(*(::core::ptr::null::<cdrom_ti>())).cdti_ind0 as *const _ as usize },
            stringify!(cdrom_ti),
        unsafe { &(*(::core::ptr::null::<cdrom_ti>())).cdti_trk1 as *const _ as usize },
            stringify!(cdrom_ti),
        unsafe { &(*(::core::ptr::null::<cdrom_ti>())).cdti_ind1 as *const _ as usize },
            stringify!(cdrom_ti),
pub struct cdrom_tochdr {
fn bindgen_test_layout_cdrom_tochdr() {
        ::core::mem::size_of::<cdrom_tochdr>(),
        concat!("Size of: ", stringify!(cdrom_tochdr))
        ::core::mem::align_of::<cdrom_tochdr>(),
        concat!("Alignment of ", stringify!(cdrom_tochdr))
        unsafe { &(*(::core::ptr::null::<cdrom_tochdr>())).cdth_trk0 as *const _ as usize },
            stringify!(cdrom_tochdr),
        unsafe { &(*(::core::ptr::null::<cdrom_tochdr>())).cdth_trk1 as *const _ as usize },
            stringify!(cdrom_tochdr),
pub struct cdrom_volctrl {
fn bindgen_test_layout_cdrom_volctrl() {
        ::core::mem::size_of::<cdrom_volctrl>(),
        concat!("Size of: ", stringify!(cdrom_volctrl))
        ::core::mem::align_of::<cdrom_volctrl>(),
        concat!("Alignment of ", stringify!(cdrom_volctrl))
        unsafe { &(*(::core::ptr::null::<cdrom_volctrl>())).channel0 as *const _ as usize },
            stringify!(cdrom_volctrl),
        unsafe { &(*(::core::ptr::null::<cdrom_volctrl>())).channel1 as *const _ as usize },
            stringify!(cdrom_volctrl),
        unsafe { &(*(::core::ptr::null::<cdrom_volctrl>())).channel2 as *const _ as usize },
            stringify!(cdrom_volctrl),
        unsafe { &(*(::core::ptr::null::<cdrom_volctrl>())).channel3 as *const _ as usize },
            stringify!(cdrom_volctrl),
pub struct cdrom_subchnl {
    pub cdsc_absaddr: cdrom_addr,
    pub cdsc_reladdr: cdrom_addr,
fn bindgen_test_layout_cdrom_subchnl() {
        ::core::mem::size_of::<cdrom_subchnl>(),
        concat!("Size of: ", stringify!(cdrom_subchnl))
        ::core::mem::align_of::<cdrom_subchnl>(),
        concat!("Alignment of ", stringify!(cdrom_subchnl))
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_format as *const _ as usize },
            stringify!(cdrom_subchnl),
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_audiostatus as *const _ as usize },
            stringify!(cdrom_subchnl),
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_trk as *const _ as usize },
            stringify!(cdrom_subchnl),
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_ind as *const _ as usize },
            stringify!(cdrom_subchnl),
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_absaddr as *const _ as usize },
            stringify!(cdrom_subchnl),
        unsafe { &(*(::core::ptr::null::<cdrom_subchnl>())).cdsc_reladdr as *const _ as usize },
            stringify!(cdrom_subchnl),
impl Default for cdrom_subchnl {
impl cdrom_subchnl {
pub struct cdrom_tocentry {
    pub cdte_addr: cdrom_addr,
fn bindgen_test_layout_cdrom_tocentry() {
        ::core::mem::size_of::<cdrom_tocentry>(),
        concat!("Size of: ", stringify!(cdrom_tocentry))
        ::core::mem::align_of::<cdrom_tocentry>(),
        concat!("Alignment of ", stringify!(cdrom_tocentry))
        unsafe { &(*(::core::ptr::null::<cdrom_tocentry>())).cdte_track as *const _ as usize },
            stringify!(cdrom_tocentry),
        unsafe { &(*(::core::ptr::null::<cdrom_tocentry>())).cdte_format as *const _ as usize },
            stringify!(cdrom_tocentry),
        unsafe { &(*(::core::ptr::null::<cdrom_tocentry>())).cdte_addr as *const _ as usize },
            stringify!(cdrom_tocentry),
        unsafe { &(*(::core::ptr::null::<cdrom_tocentry>())).cdte_datamode as *const _ as usize },
            stringify!(cdrom_tocentry),
impl Default for cdrom_tocentry {
impl cdrom_tocentry {
pub struct cdrom_read {
fn bindgen_test_layout_cdrom_read() {
        ::core::mem::size_of::<cdrom_read>(),
        concat!("Size of: ", stringify!(cdrom_read))
        ::core::mem::align_of::<cdrom_read>(),
        concat!("Alignment of ", stringify!(cdrom_read))
        unsafe { &(*(::core::ptr::null::<cdrom_read>())).cdread_lba as *const _ as usize },
            stringify!(cdrom_read),
        unsafe { &(*(::core::ptr::null::<cdrom_read>())).cdread_bufaddr as *const _ as usize },
            stringify!(cdrom_read),
        unsafe { &(*(::core::ptr::null::<cdrom_read>())).cdread_buflen as *const _ as usize },
            stringify!(cdrom_read),
impl Default for cdrom_read {
pub struct cdrom_read_audio {
    pub addr: cdrom_addr,
fn bindgen_test_layout_cdrom_read_audio() {
        ::core::mem::size_of::<cdrom_read_audio>(),
        concat!("Size of: ", stringify!(cdrom_read_audio))
        ::core::mem::align_of::<cdrom_read_audio>(),
        concat!("Alignment of ", stringify!(cdrom_read_audio))
        unsafe { &(*(::core::ptr::null::<cdrom_read_audio>())).addr as *const _ as usize },
            stringify!(cdrom_read_audio),
        unsafe { &(*(::core::ptr::null::<cdrom_read_audio>())).addr_format as *const _ as usize },
            stringify!(cdrom_read_audio),
        unsafe { &(*(::core::ptr::null::<cdrom_read_audio>())).nframes as *const _ as usize },
            stringify!(cdrom_read_audio),
        unsafe { &(*(::core::ptr::null::<cdrom_read_audio>())).buf as *const _ as usize },
            stringify!(cdrom_read_audio),
impl Default for cdrom_read_audio {
pub struct cdrom_multisession {
    pub addr: cdrom_addr,
fn bindgen_test_layout_cdrom_multisession() {
        ::core::mem::size_of::<cdrom_multisession>(),
        concat!("Size of: ", stringify!(cdrom_multisession))
        ::core::mem::align_of::<cdrom_multisession>(),
        concat!("Alignment of ", stringify!(cdrom_multisession))
        unsafe { &(*(::core::ptr::null::<cdrom_multisession>())).addr as *const _ as usize },
            stringify!(cdrom_multisession),
        unsafe { &(*(::core::ptr::null::<cdrom_multisession>())).xa_flag as *const _ as usize },
            stringify!(cdrom_multisession),
        unsafe { &(*(::core::ptr::null::<cdrom_multisession>())).addr_format as *const _ as usize },
            stringify!(cdrom_multisession),
impl Default for cdrom_multisession {
pub struct cdrom_mcn {
fn bindgen_test_layout_cdrom_mcn() {
        ::core::mem::size_of::<cdrom_mcn>(),
        concat!("Size of: ", stringify!(cdrom_mcn))
        ::core::mem::align_of::<cdrom_mcn>(),
        concat!("Alignment of ", stringify!(cdrom_mcn))
            &(*(::core::ptr::null::<cdrom_mcn>())).medium_catalog_number as *const _ as usize
            stringify!(cdrom_mcn),
pub struct cdrom_blk {
fn bindgen_test_layout_cdrom_blk() {
        ::core::mem::size_of::<cdrom_blk>(),
        concat!("Size of: ", stringify!(cdrom_blk))
        ::core::mem::align_of::<cdrom_blk>(),
        concat!("Alignment of ", stringify!(cdrom_blk))
        unsafe { &(*(::core::ptr::null::<cdrom_blk>())).from as *const _ as usize },
            stringify!(cdrom_blk),
        unsafe { &(*(::core::ptr::null::<cdrom_blk>())).len as *const _ as usize },
            stringify!(cdrom_blk),
pub struct cdrom_generic_command {
    pub __bindgen_anon_1: cdrom_generic_command__bindgen_ty_1,
pub union cdrom_generic_command__bindgen_ty_1 {
fn bindgen_test_layout_cdrom_generic_command__bindgen_ty_1() {
        ::core::mem::size_of::<cdrom_generic_command__bindgen_ty_1>(),
        concat!("Size of: ", stringify!(cdrom_generic_command__bindgen_ty_1))
        ::core::mem::align_of::<cdrom_generic_command__bindgen_ty_1>(),
            stringify!(cdrom_generic_command__bindgen_ty_1)
            &(*(::core::ptr::null::<cdrom_generic_command__bindgen_ty_1>())).reserved as *const _
            stringify!(cdrom_generic_command__bindgen_ty_1),
            &(*(::core::ptr::null::<cdrom_generic_command__bindgen_ty_1>())).unused as *const _
            stringify!(cdrom_generic_command__bindgen_ty_1),
impl Default for cdrom_generic_command__bindgen_ty_1 {
fn bindgen_test_layout_cdrom_generic_command() {
        ::core::mem::size_of::<cdrom_generic_command>(),
        concat!("Size of: ", stringify!(cdrom_generic_command))
        ::core::mem::align_of::<cdrom_generic_command>(),
        concat!("Alignment of ", stringify!(cdrom_generic_command))
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).cmd as *const _ as usize },
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).buffer as *const _ as usize },
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).buflen as *const _ as usize },
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).stat as *const _ as usize },
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).sense as *const _ as usize },
            stringify!(cdrom_generic_command),
            &(*(::core::ptr::null::<cdrom_generic_command>())).data_direction as *const _ as usize
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).quiet as *const _ as usize },
            stringify!(cdrom_generic_command),
        unsafe { &(*(::core::ptr::null::<cdrom_generic_command>())).timeout as *const _ as usize },
            stringify!(cdrom_generic_command),
impl Default for cdrom_generic_command {
pub struct cdrom_timed_media_change_info {
fn bindgen_test_layout_cdrom_timed_media_change_info() {
        ::core::mem::size_of::<cdrom_timed_media_change_info>(),
        concat!("Size of: ", stringify!(cdrom_timed_media_change_info))
        ::core::mem::align_of::<cdrom_timed_media_change_info>(),
        concat!("Alignment of ", stringify!(cdrom_timed_media_change_info))
            &(*(::core::ptr::null::<cdrom_timed_media_change_info>())).last_media_change as *const _
            stringify!(cdrom_timed_media_change_info),
            &(*(::core::ptr::null::<cdrom_timed_media_change_info>())).media_flags as *const _
            stringify!(cdrom_timed_media_change_info),
pub struct cdrom_device_info {
    pub ops: *const cdrom_device_ops,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info) -> c_types::c_int,
fn bindgen_test_layout_cdrom_device_info() {
        ::core::mem::size_of::<cdrom_device_info>(),
        concat!("Size of: ", stringify!(cdrom_device_info))
        ::core::mem::align_of::<cdrom_device_info>(),
        concat!("Alignment of ", stringify!(cdrom_device_info))
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).ops as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).list as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).disk as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).handle as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).mask as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).speed as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).capacity as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).vfs_events as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).ioctl_events as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).use_count as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).name as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).cdda_method as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).last_sense as *const _ as usize },
            stringify!(cdrom_device_info),
            &(*(::core::ptr::null::<cdrom_device_info>())).media_written as *const _ as usize
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).mmc3_profile as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).for_data as *const _ as usize },
            stringify!(cdrom_device_info),
        unsafe { &(*(::core::ptr::null::<cdrom_device_info>())).exit as *const _ as usize },
            stringify!(cdrom_device_info),
            &(*(::core::ptr::null::<cdrom_device_info>())).mrw_mode_page as *const _ as usize
            stringify!(cdrom_device_info),
            &(*(::core::ptr::null::<cdrom_device_info>())).last_media_change_ms as *const _ as usize
            stringify!(cdrom_device_info),
impl Default for cdrom_device_info {
impl cdrom_device_info {
pub struct cdrom_device_ops {
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
    pub release: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cdrom_device_info)>,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
            cdi: *mut cdrom_device_info,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: c_types::c_int) -> c_types::c_int,
            arg1: *mut cdrom_device_info,
            arg2: *mut cdrom_multisession,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info, arg2: *mut cdrom_mcn) -> c_types::c_int,
        unsafe extern "C" fn(arg1: *mut cdrom_device_info) -> c_types::c_int,
            arg1: *mut cdrom_device_info,
            arg1: *mut cdrom_device_info,
            cdi: *mut cdrom_device_info,
fn bindgen_test_layout_cdrom_device_ops() {
        ::core::mem::size_of::<cdrom_device_ops>(),
        concat!("Size of: ", stringify!(cdrom_device_ops))
        ::core::mem::align_of::<cdrom_device_ops>(),
        concat!("Alignment of ", stringify!(cdrom_device_ops))
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).open as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).release as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).drive_status as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).check_events as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).tray_move as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).lock_door as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).select_speed as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).select_disc as *const _ as usize },
            stringify!(cdrom_device_ops),
            &(*(::core::ptr::null::<cdrom_device_ops>())).get_last_session as *const _ as usize
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).get_mcn as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).reset as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).audio_ioctl as *const _ as usize },
            stringify!(cdrom_device_ops),
            &(*(::core::ptr::null::<cdrom_device_ops>())).generic_packet as *const _ as usize
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).read_cdda_bpc as *const _ as usize },
            stringify!(cdrom_device_ops),
        unsafe { &(*(::core::ptr::null::<cdrom_device_ops>())).capability as *const _ as usize },
            stringify!(cdrom_device_ops),
    pub fn cdrom_multisession(
        cdi: *mut cdrom_device_info,
        info: *mut cdrom_multisession,
    pub fn cdrom_read_tocentry(
        cdi: *mut cdrom_device_info,
        entry: *mut cdrom_tocentry,
    pub fn cdrom_open(
        cdi: *mut cdrom_device_info,
    pub fn cdrom_release(cdi: *mut cdrom_device_info, mode: fmode_t);
    pub fn cdrom_ioctl(
        cdi: *mut cdrom_device_info,
    pub fn cdrom_check_events(
        cdi: *mut cdrom_device_info,
    pub fn register_cdrom(disk: *mut gendisk, cdi: *mut cdrom_device_info) -> c_types::c_int;
    pub fn unregister_cdrom(cdi: *mut cdrom_device_info);
    pub fn cdrom_get_last_written(
        cdi: *mut cdrom_device_info,
    pub fn cdrom_number_of_slots(cdi: *mut cdrom_device_info) -> c_types::c_int;
    pub fn cdrom_mode_select(
        cdi: *mut cdrom_device_info,
    pub fn cdrom_mode_sense(
        cdi: *mut cdrom_device_info,
    pub fn init_cdrom_command(
    pub fn cdrom_dummy_generic_packet(
        cdi: *mut cdrom_device_info,
pub struct cdrom_mechstat_header {
fn bindgen_test_layout_cdrom_mechstat_header() {
        ::core::mem::size_of::<cdrom_mechstat_header>(),
        concat!("Size of: ", stringify!(cdrom_mechstat_header))
        ::core::mem::align_of::<cdrom_mechstat_header>(),
        concat!("Alignment of ", stringify!(cdrom_mechstat_header))
        unsafe { &(*(::core::ptr::null::<cdrom_mechstat_header>())).curlba as *const _ as usize },
            stringify!(cdrom_mechstat_header),
        unsafe { &(*(::core::ptr::null::<cdrom_mechstat_header>())).nslots as *const _ as usize },
            stringify!(cdrom_mechstat_header),
            &(*(::core::ptr::null::<cdrom_mechstat_header>())).slot_tablelen as *const _ as usize
            stringify!(cdrom_mechstat_header),
impl cdrom_mechstat_header {
pub struct cdrom_slot {
fn bindgen_test_layout_cdrom_slot() {
        ::core::mem::size_of::<cdrom_slot>(),
        concat!("Size of: ", stringify!(cdrom_slot))
        ::core::mem::align_of::<cdrom_slot>(),
        concat!("Alignment of ", stringify!(cdrom_slot))
        unsafe { &(*(::core::ptr::null::<cdrom_slot>())).reserved2 as *const _ as usize },
            stringify!(cdrom_slot),
impl cdrom_slot {
pub struct cdrom_changer_info {
    pub hdr: cdrom_mechstat_header,
    pub slots: [cdrom_slot; 256usize],
fn bindgen_test_layout_cdrom_changer_info() {
        ::core::mem::size_of::<cdrom_changer_info>(),
        concat!("Size of: ", stringify!(cdrom_changer_info))
        ::core::mem::align_of::<cdrom_changer_info>(),
        concat!("Alignment of ", stringify!(cdrom_changer_info))
        unsafe { &(*(::core::ptr::null::<cdrom_changer_info>())).hdr as *const _ as usize },
            stringify!(cdrom_changer_info),
        unsafe { &(*(::core::ptr::null::<cdrom_changer_info>())).slots as *const _ as usize },
            stringify!(cdrom_changer_info),
impl Default for cdrom_changer_info {
    pub fn cdrom_get_media_event(
        cdi: *mut cdrom_device_info,
